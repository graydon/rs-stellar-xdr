require 'xdrgen'

class Generator < Xdrgen::Generators::Base

  AST = Xdrgen::AST

  def generate
    @already_rendered = []
    path = "#{@namespace}.rs"
    out = @output.open(path)

    @types = build_type_list(@top)
    @type_field_types = build_type_field_types(@top)
    @xdr_fixed_size_cache = {}
    @xdr_fixed_size_computing = Set.new

    render_top_matter(out)
    render_lib(out)
    render_definitions(out, @top)
    render_enum_of_all_types(out, @types)
    out.break

    # Generate zero-copy buffer reference types.
    @already_rendered_ref = []
    ref_path = "#{@namespace}_refs.rs"
    ref_out = @output.open(ref_path)
    render_ref_top_matter(ref_out)
    render_ref_lib(ref_out)
    render_definitions_ref(ref_out, @top)
    render_enum_of_all_types_ref(ref_out, @types)
    ref_out.break
  end

  private

  def build_type_list(node)
    types = Set.new
    ingest_node = lambda do |n|
      case n
      when AST::Definitions::Struct, AST::Definitions::Enum, AST::Definitions::Union, AST::Definitions::Typedef
        types << name(n)
      end
      n.definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:definitions)
      n.nested_definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:nested_definitions)
    end
    ingest_node.call(node)
    types
  end

  def build_type_field_types(node)
    types = Hash.new { |h, k| h[k] = [] }
    ingest_node = lambda do |n|
      n.definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:definitions)
      n.nested_definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:nested_definitions)
      case n
      when AST::Definitions::Struct
        n.members.each do |m|
          types[name(n)] << base_reference(m.declaration.type)
        end
      when AST::Definitions::Union ;
        union_cases(n) do |_, arm|
          types[name(n)] << base_reference(arm.type) unless arm.void?
        end
      end
    end
    ingest_node.call(node)
    types
  end

  # Determines if 'type' is referenced directly or indirectly by 'type_with_fields'.
  # Used to determine if 'type_with_fields' has a recursive relationship to 'type'.
  def is_type_in_type_field_types(type_with_fields, type, seen = [])
    return false if seen.include?(type_with_fields)
    seen << type_with_fields
    @type_field_types[type_with_fields].any? do |field_type|
      if field_type == type
        true
      else
        is_type_in_type_field_types(field_type, type, seen)
      end
    end
  end

  def render_top_matter(out)
    out.puts <<-EOS.strip_heredoc
      // Module #{@namepsace} is generated from:
      //  #{@output.relative_source_paths.join("\n//  ")}
    EOS
    out.break
    out.puts "#![allow(clippy::missing_errors_doc, clippy::unreadable_literal)]"
    out.break
    source_paths_sha256_hashes = @output.relative_source_path_sha256_hashes
    out.puts <<-EOS.strip_heredoc
      /// `XDR_FILES_SHA256` is a list of pairs of source files and their SHA256 hashes.
      pub const XDR_FILES_SHA256: [(&str, &str); #{source_paths_sha256_hashes.count}] = [
        #{source_paths_sha256_hashes.map(){ |path, hash| %{("#{path}", "#{hash}")} }.join(",\n")}
      ];
    EOS
    out.break
  end

  def render_lib(out)
    header = IO.read(__dir__ + "/header.rs")
    out.puts(header)
    out.break
    header_refs = IO.read(__dir__ + "/header_refs.rs")
    out.puts(header_refs)
    out.break
  end

  def render_enum_of_all_types(out, types)
    out.puts <<-EOS.strip_heredoc
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[cfg_attr(
      all(feature = "serde", feature = "alloc"),
      derive(serde::Serialize, serde::Deserialize),
      serde(rename_all = "snake_case")
    )]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    pub enum TypeVariant {
        #{types.map { |t| "#{t}," }.join("\n")}
    }

    impl TypeVariant {
        pub const VARIANTS: [TypeVariant; #{types.count}] = [ #{types.map { |t| "TypeVariant::#{t}," }.join("\n")} ];
        pub const VARIANTS_STR: [&'static str; #{types.count}] = [ #{types.map { |t| "\"#{t}\"," }.join("\n")} ];

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn name(&self) -> &'static str {
            match self {
                #{types.map { |t| "Self::#{t} => \"#{t}\"," }.join("\n")}
            }
        }

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn variants() -> [TypeVariant; #{types.count}] {
            Self::VARIANTS
        }

        #[cfg(feature = "schemars")]
        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub fn json_schema(&self, gen: schemars::gen::SchemaGenerator) -> schemars::schema::RootSchema {
            match self {
                #{types.map { |t| "Self::#{t} => gen.into_root_schema_for::<#{t}>()," }.join("\n")}
            }
        }
    }

    impl Name for TypeVariant {
        #[must_use]
        fn name(&self) -> &'static str {
            Self::name(self)
        }
    }

    impl Variants<TypeVariant> for TypeVariant {
        fn variants() -> slice::Iter<'static, TypeVariant> {
            Self::VARIANTS.iter()
        }
    }

    impl core::str::FromStr for TypeVariant {
        type Err = Error;
        #[allow(clippy::too_many_lines)]
        fn from_str(s: &str) -> Result<Self, Error> {
            match s {
                #{types.map { |t| "\"#{t}\" => Ok(Self::#{t})," }.join("\n")}
                _ => Err(Error::Invalid),
            }
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[cfg_attr(
      all(feature = "serde", feature = "alloc"),
      derive(serde::Serialize, serde::Deserialize),
      serde(rename_all = "snake_case"),
      serde(untagged),
    )]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    pub enum Type {
        #{types.map { |t| "#{t}(Box<#{t}>)," }.join("\n")}
    }

    impl Type {
        pub const VARIANTS: [TypeVariant; #{types.count}] = [ #{types.map { |t| "TypeVariant::#{t}," }.join("\n")} ];
        pub const VARIANTS_STR: [&'static str; #{types.count}] = [ #{types.map { |t| "\"#{t}\"," }.join("\n")} ];

        #[cfg(feature = "std")]
        #[allow(clippy::too_many_lines)]
        pub fn read_xdr<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Result<Self, Error> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => r.with_limited_depth(|r| Ok(Self::#{t}(Box::new(#{t}::read_xdr(r)?))))," }.join("\n")}
            }
        }

        #[cfg(feature = "base64")]
        pub fn read_xdr_base64<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Result<Self, Error> {
            let mut dec = Limited::new(
                base64::read::DecoderReader::new(
                    SkipWhitespace::new(&mut r.inner),
                    &base64::engine::general_purpose::STANDARD,
                ),
                r.limits.clone(),
            );
            let t = Self::read_xdr(v, &mut dec)?;
            Ok(t)
        }

        #[cfg(feature = "std")]
        pub fn read_xdr_to_end<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Result<Self, Error> {
            let s = Self::read_xdr(v, r)?;
            // Check that any further reads, such as this read of one byte, read no
            // data, indicating EOF. If a byte is read the data is invalid.
            if r.read(&mut [0u8; 1])? == 0 {
                Ok(s)
            } else {
                Err(Error::Invalid)
            }
        }

        #[cfg(feature = "base64")]
        pub fn read_xdr_base64_to_end<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Result<Self, Error> {
            let mut dec = Limited::new(
                base64::read::DecoderReader::new(
                    SkipWhitespace::new(&mut r.inner),
                    &base64::engine::general_purpose::STANDARD,
                ),
                r.limits.clone(),
            );
            let t = Self::read_xdr_to_end(v, &mut dec)?;
            Ok(t)
        }

        #[cfg(feature = "std")]
        #[allow(clippy::too_many_lines)]
        pub fn read_xdr_iter<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Box<dyn Iterator<Item=Result<Self, Error>> + '_> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Box::new(ReadXdrIter::<_, #{t}>::new(&mut r.inner, r.limits.clone()).map(|r| r.map(|t| Self::#{t}(Box::new(t)))))," }.join("\n")}
            }
        }

        #[cfg(feature = "std")]
        #[allow(clippy::too_many_lines)]
        pub fn read_xdr_framed_iter<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Box<dyn Iterator<Item=Result<Self, Error>> + '_> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Box::new(ReadXdrIter::<_, Frame<#{t}>>::new(&mut r.inner, r.limits.clone()).map(|r| r.map(|t| Self::#{t}(Box::new(t.0)))))," }.join("\n")}
            }
        }

        #[cfg(feature = "base64")]
        #[allow(clippy::too_many_lines)]
        pub fn read_xdr_base64_iter<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Box<dyn Iterator<Item=Result<Self, Error>> + '_> {
            let dec = base64::read::DecoderReader::new(
                SkipWhitespace::new(&mut r.inner),
                &base64::engine::general_purpose::STANDARD,
            );
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Box::new(ReadXdrIter::<_, #{t}>::new(dec, r.limits.clone()).map(|r| r.map(|t| Self::#{t}(Box::new(t)))))," }.join("\n")}
            }
        }

        #[cfg(feature = "std")]
        pub fn from_xdr<B: AsRef<[u8]>>(v: TypeVariant, bytes: B, limits: Limits) -> Result<Self, Error> {
            let mut cursor = Limited::new(Cursor::new(bytes.as_ref()), limits);
            let t = Self::read_xdr_to_end(v, &mut cursor)?;
            Ok(t)
        }

        #[cfg(feature = "base64")]
        pub fn from_xdr_base64(v: TypeVariant, b64: impl AsRef<[u8]>, limits: Limits) -> Result<Self, Error> {
            let mut dec = Limited::new(
                base64::read::DecoderReader::new(
                    SkipWhitespace::new(Cursor::new(b64)),
                    &base64::engine::general_purpose::STANDARD,
                ),
                limits,
            );
            let t = Self::read_xdr_to_end(v, &mut dec)?;
            Ok(t)
        }

        #[cfg(all(feature = "std", feature = "serde_json"))]
        #[deprecated(note = "use from_json")]
        pub fn read_json(v: TypeVariant, r: impl Read) -> Result<Self, Error> {
            Self::from_json(v, r)
        }

        #[cfg(all(feature = "std", feature = "serde_json"))]
        #[allow(clippy::too_many_lines)]
        pub fn from_json(v: TypeVariant, r: impl Read) -> Result<Self, Error> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Ok(Self::#{t}(Box::new(serde_json::from_reader(r)?)))," }.join("\n")}
            }
        }

        #[cfg(all(feature = "std", feature = "serde_json"))]
        #[allow(clippy::too_many_lines)]
        pub fn deserialize_json<'r, R: serde_json::de::Read<'r>>(v: TypeVariant, r: &mut serde_json::de::Deserializer<R>) -> Result<Self, Error> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Ok(Self::#{t}(Box::new(serde::de::Deserialize::deserialize(r)?)))," }.join("\n")}
            }
        }

        #[cfg(feature = "arbitrary")]
        #[allow(clippy::too_many_lines)]
        pub fn arbitrary(v: TypeVariant, u: &mut arbitrary::Unstructured<'_>) -> Result<Self, Error> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Ok(Self::#{t}(Box::new(#{t}::arbitrary(u)?)))," }.join("\n")}
            }
        }

        #[cfg(feature = "alloc")]
        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub fn default(v: TypeVariant) -> Self {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Self::#{t}(Box::default())," }.join("\n")}
            }
        }

        #[cfg(feature = "alloc")]
        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub fn value(&self) -> &dyn core::any::Any {
            #[allow(clippy::match_same_arms)]
            match self {
                #{types.map { |t| "Self::#{t}(ref v) => v.as_ref()," }.join("\n")}
            }
        }

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn name(&self) -> &'static str {
            match self {
                #{types.map { |t| "Self::#{t}(_) => \"#{t}\"," }.join("\n")}
            }
        }

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn variants() -> [TypeVariant; #{types.count}] {
            Self::VARIANTS
        }

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn variant(&self) -> TypeVariant {
            match self {
                #{types.map { |t| "Self::#{t}(_) => TypeVariant::#{t}," }.join("\n")}
            }
        }
    }

    impl Name for Type {
        #[must_use]
        fn name(&self) -> &'static str {
            Self::name(self)
        }
    }

    impl Variants<TypeVariant> for Type {
        fn variants() -> slice::Iter<'static, TypeVariant> {
            Self::VARIANTS.iter()
        }
    }

    impl WriteXdr for Type {
        #[cfg(feature = "std")]
        #[allow(clippy::too_many_lines)]
        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
            match self {
                #{types.map { |t| "Self::#{t}(v) => v.write_xdr(w)," }.join("\n")}
            }
        }
    }
    EOS
    out.break
  end

  def render_definitions(out, node)
    node.definitions.each{|n| render_definition out, n }
    node.namespaces.each{|n| render_definitions out, n }
  end

  def render_definition(out, defn)
    if @already_rendered.include? name(defn)

      unless defn.is_a?(AST::Definitions::Namespace)
        $stderr.puts "warn: #{name(defn)} is defined twice.  skipping"
      end

      return
    end

    render_nested_definitions(out, defn)
    render_source_comment(out, defn)

    @already_rendered << name(defn)

    case defn
    when AST::Definitions::Struct ;
      render_struct out, defn
    when AST::Definitions::Enum ;
      render_enum out, defn
    when AST::Definitions::Union ;
      render_union out, defn
    when AST::Definitions::Typedef ;
      render_typedef out, defn
    when AST::Definitions::Const ;
      render_const out, defn
    end
  end

  def render_nested_definitions(out, defn)
    return unless defn.respond_to? :nested_definitions
    defn.nested_definitions.each{|ndefn| render_definition out, ndefn}
  end

  def render_source_comment(out, defn)
    return if defn.is_a?(AST::Definitions::Namespace)

    out.puts <<-EOS.strip_heredoc
      /// #{name defn} is an XDR #{defn.class.name.demodulize} defined as:
      ///
      /// ```text
    EOS

    out.puts "/// " + defn.text_value.split("\n").join("\n/// ")

    out.puts <<-EOS.strip_heredoc
      /// ```
      ///
    EOS
  end

  def render_struct(out, struct)
    out.puts %{#[cfg_attr(feature = "alloc", derive(Default))]} if !@options[:custom_default_impl].include?(name struct)
    out.puts "#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"
    out.puts %{#[cfg_eval::cfg_eval]}
    out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
    if @options[:custom_str_impl].include?(name struct)
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay))]}
    else
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
    end
    if !@options[:custom_str_impl].include?(name struct)
      out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
    end
    out.puts "pub struct #{name struct} {"
    out.indent do
      struct.members.each do |m|
        out.puts_if(field_attrs(struct, m.declaration.type)) if !@options[:custom_str_impl].include?(name struct)
        out.puts "pub #{field_name m}: #{reference(struct, m.declaration.type)},"
      end
    end
    out.puts "}"
    out.puts ""
    out.puts <<-EOS.strip_heredoc
    impl ReadXdr for #{name struct} {
        #[cfg(feature = "std")]
        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
            r.with_limited_depth(|r| {
                Ok(Self{
                  #{struct.members.map do |m|
                    "#{field_name(m)}: #{reference_to_call(struct, m.declaration.type)}::read_xdr(r)?,"
                  end.join("\n")}
                })
            })
        }
    }

    impl WriteXdr for #{name struct} {
        #[cfg(feature = "std")]
        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
            w.with_limited_depth(|w| {
                #{struct.members.map do |m|
                  "self.#{field_name(m)}.write_xdr(w)?;"
                end.join("\n")}
                Ok(())
            })
        }
    }
    EOS
    # Include a deserializer that will deserialize via the FromStr
    # implementation, but also deserialize the original struct, present in
    # the JSON as a map. The reason for the second option for
    # deserialization is that types that we're adding string
    # representations for were previously deserializable via a map to their
    # struct, and so this improves the backwards compatibility.
    # Note that this is only done for structs and not other types (typedef,
    # enum, union), because struct is the only type that maps to JSON in a
    # way that is unambiguous with a secondary form in string type.
    out.puts <<-EOS.strip_heredoc if @options[:custom_str_impl].include?(name struct)
    #[cfg(all(feature = "serde", feature = "alloc"))]
    impl<'de> serde::Deserialize<'de> for #{name struct} {
        fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
            use serde::Deserialize;
            #[derive(Deserialize)]
            struct #{name struct} {
                #{struct.members.map do |m|
                  "#{field_name(m)}: #{reference(struct, m.declaration.type)},"
                end.join("\n")}
            }
            #[derive(Deserialize)]
            #[serde(untagged)]
            enum #{name struct}OrString<'a> {
                Str(&'a str),
                String(String),
                #{name struct}(#{name struct}),
            }
            match #{name struct}OrString::deserialize(deserializer)? {
                #{name struct}OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
                #{name struct}OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
                #{name struct}OrString::#{name struct}(#{name struct} {
                    #{struct.members.map do |m| "#{field_name(m)}," end.join(" ")}
                }) => Ok(self::#{name struct} {
                    #{struct.members.map do |m| "#{field_name(m)}," end.join(" ")}
                }),
            }
        }
    }
    EOS
    out.break
  end

  def render_enum(out, enum)
    out.puts "// enum"
    out.puts %{#[cfg_attr(feature = "alloc", derive(Default))]} if !@options[:custom_default_impl].include?(name enum)
    out.puts "#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"
    out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
    if @options[:custom_str_impl].include?(name enum)
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]}
    else
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
    end
    if !@options[:custom_str_impl].include?(name enum)
      out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
    end
    out.puts "#[repr(i32)]"
    out.puts "pub enum #{name enum} {"
    out.indent do
      enum.members.each_with_index do |m, i|
        out.puts(%{#[cfg_attr(feature = "alloc", default)]}) if i == 0
        out.puts "#{name m} = #{m.value},"
      end
    end
    out.puts '}'
    out.puts ""
    out.puts <<-EOS.strip_heredoc
    impl #{name enum} {
        pub const VARIANTS: [#{name enum}; #{enum.members.count}] = [ #{enum.members.map { |m| "#{name enum}::#{name m}," }.join("\n")} ];
        pub const VARIANTS_STR: [&'static str; #{enum.members.count}] = [ #{enum.members.map { |m| "\"#{name m}\"," }.join("\n")} ];

        #[must_use]
        pub const fn name(&self) -> &'static str {
            match self {
                #{enum.members.map do |m|
                  "Self::#{name m} => \"#{name m}\","
                end.join("\n")}
            }
        }

        #[must_use]
        pub const fn variants() -> [#{name enum}; #{enum.members.count}] {
            Self::VARIANTS
        }
    }

    impl Name for #{name enum} {
        #[must_use]
        fn name(&self) -> &'static str {
            Self::name(self)
        }
    }

    impl Variants<#{name enum}> for #{name enum} {
        fn variants() -> slice::Iter<'static, #{name enum}> {
            Self::VARIANTS.iter()
        }
    }

    impl Enum for #{name enum} {}

    impl fmt::Display for #{name enum} {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.name())
        }
    }

    impl TryFrom<i32> for #{name enum} {
        type Error = Error;

        fn try_from(i: i32) -> Result<Self, Error> {
            let e = match i {
                #{enum.members.map do |m| "#{m.value} => #{name enum}::#{name m}," end.join("\n")}
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(e)
        }
    }

    impl From<#{name enum}> for i32 {
        #[must_use]
        fn from(e: #{name enum}) -> Self {
            e as Self
        }
    }

    impl ReadXdr for #{name enum} {
        #[cfg(feature = "std")]
        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
            r.with_limited_depth(|r| {
                let e = i32::read_xdr(r)?;
                let v: Self = e.try_into()?;
                Ok(v)
            })
        }
    }

    impl WriteXdr for #{name enum} {
        #[cfg(feature = "std")]
        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
            w.with_limited_depth(|w| {
                let i: i32 = (*self).into();
                i.write_xdr(w)
            })
        }
    }
    EOS
    out.break
  end

  def union_is_idents(union)
    union.normal_arms.first&.cases.first&.value.is_a?(AST::Identifier)
  end

  def union_cases(union)
    results = []
    union.normal_arms.each do |arm|
      arm.cases.each do |kase|
          if kase.value.is_a?(AST::Identifier)
            case_name = kase.name_short.underscore.camelize
            value = nil
          else
            case_name = "V#{kase.value.value}"
            value = kase.value.value
          end
          results << yield(case_name, arm, value)
      end
    end
    results
  end

  def render_union(out, union)
    if union.default_arm.present?
      $stderr.puts "warn: union #{name union} includes default arms and default arms are not supported in the rust generator"
    end
    discriminant_type = reference(nil, union.discriminant.type)
    discriminant_type_builtin = is_builtin_type(union.discriminant.type) || (is_builtin_type(union.discriminant.type.resolved_type.type) if union.discriminant.type.respond_to?(:resolved_type) && AST::Definitions::Typedef === union.discriminant.type.resolved_type)
    out.puts "// union with discriminant #{discriminant_type}"
    out.puts %{#[cfg_eval::cfg_eval]}
    out.puts "#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"
    out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
    if @options[:custom_str_impl].include?(name union)
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]}
    else
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
    end
    if !@options[:custom_str_impl].include?(name union)
      out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
    end
    out.puts "#[allow(clippy::large_enum_variant)]"
    out.puts "pub enum #{name union} {"
    union_case_count = 0
    out.indent do
      union_cases(union) do |case_name, arm|
        union_case_count += 1
        if arm.void?
          out.puts "#{case_name}#{"(())" unless arm.void?},"
        else
          out.puts "#{case_name}("
          out.puts_if(field_attrs(union, arm.type)) if !@options[:custom_str_impl].include?(name union)
          out.puts "  #{reference(union, arm.type)}"
          out.puts "),"
        end
      end
    end
    out.puts '}'
    out.puts ""
    if !@options[:custom_default_impl].include?(name union)
      union_cases(union) do |case_name, arm|
        out.puts <<-EOS.strip_heredoc
        #[cfg(feature = "alloc")]
        impl Default for #{name union} {
            fn default() -> Self {
                Self::#{case_name}#{"(#{reference_to_call(union, arm.type)}::default())" if !arm.void?}
            }
        }
        EOS
        break # output the above for the first union case
      end
      out.puts ""
    end
    out.puts <<-EOS.strip_heredoc
    impl #{name union} {
        pub const VARIANTS: [#{discriminant_type}; #{union_case_count}] = [
            #{union_cases(union) do |case_name, arm, value|
              value.nil?                ? "#{discriminant_type}::#{case_name}," :
              discriminant_type_builtin ? "#{value}," :
                                          "#{discriminant_type}(#{value}),"
            end.join("\n")}
        ];
        pub const VARIANTS_STR: [&'static str; #{union_case_count}] = [
            #{union_cases(union) do |case_name, arm, value|
              "\"#{case_name}\","
            end.join("\n")}
        ];

        #[must_use]
        pub const fn name(&self) -> &'static str {
            match self {
                #{union_cases(union) do |case_name, arm|
                  "Self::#{case_name}#{"(_)" unless arm.void?} => \"#{case_name}\","
                end.join("\n")}
            }
        }

        #[must_use]
        pub const fn discriminant(&self) -> #{discriminant_type} {
            #[allow(clippy::match_same_arms)]
            match self {
                #{union_cases(union) do |case_name, arm, value|
                  "Self::#{case_name}#{"(_)" unless arm.void?} => #{
                    value.nil?                ? "#{discriminant_type}::#{case_name}" :
                    discriminant_type_builtin ? "#{value}" :
                                                "#{discriminant_type}(#{value})"
                  },"
                end.join("\n")}
            }
        }

        #[must_use]
        pub const fn variants() -> [#{discriminant_type}; #{union_case_count}] {
            Self::VARIANTS
        }
    }

    impl Name for #{name union} {
        #[must_use]
        fn name(&self) -> &'static str {
            Self::name(self)
        }
    }

    impl Discriminant<#{discriminant_type}> for #{name union} {
        #[must_use]
        fn discriminant(&self) -> #{discriminant_type} {
            Self::discriminant(self)
        }
    }

    impl Variants<#{discriminant_type}> for #{name union} {
        fn variants() -> slice::Iter<'static, #{discriminant_type}> {
            Self::VARIANTS.iter()
        }
    }

    impl Union<#{discriminant_type}> for #{name union} {}

    impl ReadXdr for #{name union} {
        #[cfg(feature = "std")]
        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
            r.with_limited_depth(|r| {
                let dv: #{discriminant_type} = <#{discriminant_type} as ReadXdr>::read_xdr(r)?;
                #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
                let v = match dv {
                    #{union_cases(union) do |case_name, arm, value|
                      "#{
                        value.nil? ? "#{discriminant_type}::#{case_name}" : "#{value}"
                      } => #{
                        arm.void? ? "Self::#{case_name}" : "Self::#{case_name}(#{reference_to_call(union, arm.type)}::read_xdr(r)?)"
                      },"
                    end.join("\n")}
                    #[allow(unreachable_patterns)]
                    _ => return Err(Error::Invalid),
                };
                Ok(v)
            })
        }
    }

    impl WriteXdr for #{name union} {
        #[cfg(feature = "std")]
        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
            w.with_limited_depth(|w| {
                self.discriminant().write_xdr(w)?;
                #[allow(clippy::match_same_arms)]
                match self {
                    #{union_cases(union) do |case_name, arm, value|
                      if arm.void?
                        "Self::#{case_name} => ().write_xdr(w)?,"
                      else
                        "Self::#{case_name}(v) => v.write_xdr(w)?,"
                      end
                    end.join("\n")}
                };
                Ok(())
            })
        }
    }
    EOS
    out.break
  end

  def render_typedef(out, typedef)
    if is_builtin_type(typedef.type)
      out.puts "pub type #{name typedef} = #{reference(typedef, typedef.type)};"
    else
      out.puts %{#[cfg_eval::cfg_eval]}
      if !@options[:custom_default_impl].include?(name typedef)
        if is_var_array_type(typedef.type)
          out.puts "#[derive(Default)]"
        else
          out.puts %{#[cfg_attr(feature = "alloc", derive(Default))]}
        end
      end
      out.puts "#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]"
      out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
      if is_fixed_array_opaque(typedef.type) || @options[:custom_str_impl].include?(name typedef)
        out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]}
      else
        out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
      end
      if !is_fixed_array_opaque(typedef.type) && !@options[:custom_str_impl].include?(name typedef)
        out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
      end
      if !is_fixed_array_opaque(typedef.type)
        out.puts "#[derive(Debug)]"
      end
      out.puts "pub struct #{name typedef}("
      out.puts_if(field_attrs(typedef, typedef.type)) if !@options[:custom_str_impl].include?(name typedef)
      out.puts "  pub #{reference(typedef, typedef.type)}"
      out.puts ");"
      out.puts ""
      if is_fixed_array_opaque(typedef.type)
      out.puts <<-EOS.strip_heredoc
      impl core::fmt::Debug for #{name typedef} {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let v = &self.0;
            write!(f, "#{name typedef}(")?;
            for b in v {
                write!(f, "{b:02x}")?;
            }
            write!(f, ")")?;
            Ok(())
        }
      }
      EOS
      end
      if is_fixed_array_opaque(typedef.type) && !@options[:custom_str_impl].include?(name typedef)
      out.puts <<-EOS.strip_heredoc
      impl core::fmt::Display for #{name typedef} {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let v = &self.0;
            for b in v {
                write!(f, "{b:02x}")?;
            }
            Ok(())
        }
      }

      #[cfg(feature = "alloc")]
      impl core::str::FromStr for #{name typedef} {
        type Err = Error;
        fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
            hex::decode(s).map_err(|_| Error::InvalidHex)?.try_into()
        }
      }
      EOS
      end
      if is_fixed_array_opaque(typedef.type) && !@options[:custom_str_impl].include?(name typedef)
      out.puts <<-EOS.strip_heredoc
      #[cfg(feature = "schemars")]
      impl schemars::JsonSchema for #{name typedef} {
          fn schema_name() -> String {
              "#{name typedef}".to_string()
          }

          fn is_referenceable() -> bool {
              false
          }

          fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
              let schema = String::json_schema(gen);
              if let schemars::schema::Schema::Object(mut schema) = schema {
                  schema.extensions.insert(
                      "contentEncoding".to_owned(),
                      serde_json::Value::String("hex".to_string()),
                  );
                  schema.extensions.insert(
                      "contentMediaType".to_owned(),
                      serde_json::Value::String("application/binary".to_string()),
                  );
                  let string = *schema.string.unwrap_or_default().clone();
                  schema.string = Some(Box::new(schemars::schema::StringValidation {
                      max_length: #{typedef.type.size}_u32.checked_mul(2).map(Some).unwrap_or_default(),
                      min_length: #{typedef.type.size}_u32.checked_mul(2).map(Some).unwrap_or_default(),
                      ..string
                  }));
                  schema.into()
              } else {
                  schema
              }
          }
      }
      EOS
      end
      out.puts <<-EOS.strip_heredoc
      impl From<#{name typedef}> for #{reference(typedef, typedef.type)} {
          #[must_use]
          fn from(x: #{name typedef}) -> Self {
              x.0
          }
      }

      impl From<#{reference(typedef, typedef.type)}> for #{name typedef} {
          #[must_use]
          fn from(x: #{reference(typedef, typedef.type)}) -> Self {
              #{name typedef}(x)
          }
      }

      impl AsRef<#{reference(typedef, typedef.type)}> for #{name typedef} {
          #[must_use]
          fn as_ref(&self) -> &#{reference(typedef, typedef.type)} {
              &self.0
          }
      }

      impl ReadXdr for #{name typedef} {
          #[cfg(feature = "std")]
          fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
              r.with_limited_depth(|r| {
                  let i = #{reference_to_call(typedef, typedef.type)}::read_xdr(r)?;
                  let v = #{name typedef}(i);
                  Ok(v)
              })
          }
      }

      impl WriteXdr for #{name typedef} {
          #[cfg(feature = "std")]
          fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
              w.with_limited_depth(|w|{ self.0.write_xdr(w) })
          }
      }
      EOS
      if is_fixed_array_type(typedef.type)
        out.break
        out.puts <<-EOS.strip_heredoc
        impl #{name typedef} {
            #[must_use]
            pub fn as_slice(&self) -> &[#{element_type_for_vec(typedef.type)}] {
                &self.0
            }
        }

        #[cfg(feature = "alloc")]
        impl TryFrom<Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            type Error = Error;
            fn try_from(x: Vec<#{element_type_for_vec(typedef.type)}>) -> Result<Self, Error> {
                x.as_slice().try_into()
            }
        }

        #[cfg(feature = "alloc")]
        impl TryFrom<&Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            type Error = Error;
            fn try_from(x: &Vec<#{element_type_for_vec(typedef.type)}>) -> Result<Self, Error> {
                x.as_slice().try_into()
            }
        }

        impl TryFrom<&[#{element_type_for_vec(typedef.type)}]> for #{name typedef} {
            type Error = Error;
            fn try_from(x: &[#{element_type_for_vec(typedef.type)}]) -> Result<Self, Error> {
                Ok(#{name typedef}(x.try_into()?))
            }
        }

        impl AsRef<[#{element_type_for_vec(typedef.type)}]> for #{name typedef} {
            #[must_use]
            fn as_ref(&self) -> &[#{element_type_for_vec(typedef.type)}] {
                &self.0
            }
        }
        EOS
      end
      if is_var_array_type(typedef.type)
        out.break
        out.puts <<-EOS.strip_heredoc
        impl Deref for #{name typedef} {
          type Target = #{reference(typedef, typedef.type)};
          fn deref(&self) -> &Self::Target {
              &self.0
          }
        }

        impl From<#{name typedef}> for Vec<#{element_type_for_vec(typedef.type)}> {
            #[must_use]
            fn from(x: #{name typedef}) -> Self {
                x.0.0
            }
        }

        impl TryFrom<Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            type Error = Error;
            fn try_from(x: Vec<#{element_type_for_vec(typedef.type)}>) -> Result<Self, Error> {
                Ok(#{name typedef}(x.try_into()?))
            }
        }

        #[cfg(feature = "alloc")]
        impl TryFrom<&Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            type Error = Error;
            fn try_from(x: &Vec<#{element_type_for_vec(typedef.type)}>) -> Result<Self, Error> {
                Ok(#{name typedef}(x.try_into()?))
            }
        }

        impl AsRef<Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            #[must_use]
            fn as_ref(&self) -> &Vec<#{element_type_for_vec(typedef.type)}> {
                &self.0.0
            }
        }

        impl AsRef<[#{element_type_for_vec(typedef.type)}]> for #{name typedef} {
            #[cfg(feature = "alloc")]
            #[must_use]
            fn as_ref(&self) -> &[#{element_type_for_vec(typedef.type)}] {
                &self.0.0
            }
            #[cfg(not(feature = "alloc"))]
            #[must_use]
            fn as_ref(&self) -> &[#{element_type_for_vec(typedef.type)}] {
                self.0.0
            }
        }
        EOS
      end
    end
    out.break
  end

  def render_const(out, const)
    out.puts "pub const #{name(const).underscore.upcase}: u64 = #{const.value};"
    out.break
  end

  def is_builtin_type(type)
    [
      AST::Typespecs::Bool,
      AST::Typespecs::Double, AST::Typespecs::Float,
      AST::Typespecs::UnsignedHyper, AST::Typespecs::UnsignedInt,
      AST::Typespecs::Hyper, AST::Typespecs::Int,
    ].any? { |t| t === type }
  end

  def is_fixed_array_opaque(type)
    (AST::Typespecs::Opaque === type && type.fixed?)
  end

  def is_fixed_array_type(type)
    (AST::Typespecs::Opaque === type && type.fixed?) ||
    (type.sub_type == :array)
  end

  def is_var_array_type(type)
    (AST::Typespecs::Opaque === type && !type.fixed?) ||
    (AST::Typespecs::String === type) ||
    (type.sub_type == :var_array)
  end

  def base_reference(type)
    case type
    when AST::Typespecs::Bool
      'bool'
    when AST::Typespecs::Double
      $stderr.puts "warn: rust generator has not implemented f64 support"
      'f64'
    when AST::Typespecs::Float
      $stderr.puts "warn: rust generator has not implemented f64 support"
      'f32'
    when AST::Typespecs::UnsignedHyper
      'u64'
    when AST::Typespecs::UnsignedInt
      'u32'
    when AST::Typespecs::Hyper
      'i64'
    when AST::Typespecs::Int
      'i32'
    when AST::Typespecs::Quadruple
      raise 'no quadruple support for rust'
    when AST::Typespecs::String
      if !type.decl.resolved_size.nil?
        "StringM::<#{type.decl.resolved_size}>"
      else
        "StringM"
      end
    when AST::Typespecs::Opaque
      if type.fixed?
        "[u8; #{type.size}]"
      elsif !type.decl.resolved_size.nil?
        "BytesM::<#{type.decl.resolved_size}>"
      else
        "BytesM"
      end
    when AST::Typespecs::Simple, AST::Definitions::Base, AST::Concerns::NestedDefinition
      if type.respond_to?(:resolved_type) && AST::Definitions::Typedef === type.resolved_type && is_builtin_type(type.resolved_type.type)
        base_reference(type.resolved_type.type)
      else
        name type
      end
    else
      raise "Unknown reference type: #{type.class.name}, #{type.class.ancestors}"
    end
  end

  def array_size(type)
    _, size = type.array_size
    size = name @top.find_definition(size) if is_named
    size
  end

  def field_attrs(parent, type)
    base_ref = base_reference(type)
    if ['i64','u64'].include?(base_ref)
      ref = reference(parent, type, 'NumberOrString')
      "#[cfg_attr(all(feature = \"serde\", feature = \"alloc\"), serde_as(as = \"#{ref}\"))]"
    else
      nil
    end
  end

  def reference(parent, type, base_ref = nil)
    base_ref = base_reference(type) if base_ref.nil?

    parent_name = name(parent) if parent
    cyclic = is_type_in_type_field_types(base_ref, parent_name)

    case type.sub_type
    when :simple
      if cyclic
        "Box<#{base_ref}>"
      else
        base_ref
      end
    when :optional
      if cyclic
        "Option<Box<#{base_ref}>>"
      else
        "Option<#{base_ref}>"
      end
    when :array
      is_named, size = type.array_size
      size = name @top.find_definition(size) if is_named
      "[#{base_ref}; #{size}]"
    when :var_array
      if !type.decl.resolved_size.nil?
        "VecM<#{base_ref}, #{type.decl.resolved_size}>"
      else
        "VecM<#{base_ref}>"
      end
    else
      raise "Unknown sub_type: #{type.sub_type}"
    end
  end

  def element_type_for_vec(type)
    case type
    when AST::Typespecs::String
      "u8"
    when AST::Typespecs::Opaque
      "u8"
    when AST::Typespecs::Simple, AST::Definitions::Base, AST::Concerns::NestedDefinition
      if type.respond_to?(:resolved_type) && AST::Definitions::Typedef === type.resolved_type && is_builtin_type(type.resolved_type.type)
        base_reference(type.resolved_type.type)
      else
        name type
      end
    else
      raise "Unknown element type for vec: #{type.class.name}, #{type.class.ancestors}"
    end
  end

  def base_reference_to_call(type)
    case type
    when AST::Typespecs::String
      if !type.decl.resolved_size.nil?
        "StringM::<#{type.decl.resolved_size}>"
      else
        "StringM"
      end
    when AST::Typespecs::Opaque
      if type.fixed?
        "[u8; #{type.size}]"
      elsif !type.decl.resolved_size.nil?
        "BytesM::<#{type.decl.resolved_size}>"
      else
        "BytesM"
      end
    when AST::Typespecs::Simple, AST::Definitions::Base, AST::Concerns::NestedDefinition
      if type.respond_to?(:resolved_type) && AST::Definitions::Typedef === type.resolved_type && is_builtin_type(type.resolved_type.type)
        base_reference_to_call(type.resolved_type.type)
      else
        base_reference(type)
      end
    else
      base_reference(type)
    end
  end

  def reference_to_call(parent, type)
    base_ref = base_reference_to_call(type)

    parent_name = name(parent) if parent
    cyclic = is_type_in_type_field_types(base_ref, parent_name)

    ref = case type.sub_type
    when :simple
      if cyclic
        "Box<#{base_ref}>"
      else
        base_ref
      end
    when :optional
      if cyclic
        "Option::<Box<#{base_ref}>>"
      else
        "Option::<#{base_ref}>"
      end
    when :array
      is_named, size = type.array_size
      size = name @top.find_definition(size) if is_named
      "[#{base_ref}; #{size}]"
    when :var_array
      if !type.decl.resolved_size.nil?
        "VecM::<#{base_ref}, #{type.decl.resolved_size}>"
      else
        "VecM::<#{base_ref}>"
      end
    else
      raise "Unknown sub_type: #{type.sub_type}"
    end

    if ref.starts_with?("[") && ref.ends_with?("]")
      "<#{ref}>"
    elsif ref.starts_with?("Box<") && ref.ends_with?(">")
      "Box::#{ref.delete_prefix("Box")}"
    else
      ref
    end
  end

  def name(named)
    parent = name named.parent_defn if named.is_a?(AST::Concerns::NestedDefinition)

    base = if named.respond_to?(:name_short)
      named.name_short
    elsif named.respond_to?(:name)
      named.name
    else
      named.text_value
    end
    base = escape_name(base)
    "#{parent}#{base.underscore.camelize}"
  end

  def field_name(named)
    escape_name named.name.underscore
  end

  def escape_name(name)
    case name
    when 'type' then 'type_'
    when 'Error' then 'SError'
    else name
    end
  end

  # ---------------------------------------------------------------------------
  # Fixed-size determination
  # ---------------------------------------------------------------------------

  # Returns the fixed XDR byte size for a type, or nil if variable.
  # Memoized and cycle-safe (cycles => nil => variable).
  def xdr_fixed_size_of_type(type)
    case type
    when AST::Typespecs::Bool then 4
    when AST::Typespecs::Int then 4
    when AST::Typespecs::UnsignedInt then 4
    when AST::Typespecs::Hyper then 8
    when AST::Typespecs::UnsignedHyper then 8
    when AST::Typespecs::Float then 4
    when AST::Typespecs::Double then 8
    when AST::Typespecs::String then nil  # variable
    when AST::Typespecs::Opaque
      if type.fixed?
        n = type.size.to_i
        n + ((4 - (n % 4)) % 4)
      else
        nil  # variable
      end
    when AST::Typespecs::Simple, AST::Definitions::Base, AST::Concerns::NestedDefinition
      if type.respond_to?(:resolved_type) && AST::Definitions::Typedef === type.resolved_type && is_builtin_type(type.resolved_type.type)
        xdr_fixed_size_of_type(type.resolved_type.type)
      elsif type.respond_to?(:resolved_type) && type.resolved_type
        xdr_fixed_size_of_defn(type.resolved_type)
      else
        nil
      end
    else
      nil
    end
  end

  # Returns the fixed XDR byte size of a field (accounts for sub_type: :simple, :optional, :array, :var_array).
  def xdr_fixed_size_of_field(type)
    case type.sub_type
    when :simple
      xdr_fixed_size_of_type(type)
    when :optional
      nil  # optional is always variable (4 bytes flag + conditionally T)
    when :array
      elem_size = xdr_fixed_size_of_type(type)
      return nil if elem_size.nil?
      is_named, size = type.array_size
      size = size.to_i
      elem_size * size
    when :var_array
      nil  # variable
    else
      nil
    end
  end

  # Returns the fixed XDR byte size of a definition (Struct, Enum, Union, Typedef), or nil.
  def xdr_fixed_size_of_defn(defn)
    n = name(defn)
    return @xdr_fixed_size_cache[n] if @xdr_fixed_size_cache.key?(n)
    return nil if @xdr_fixed_size_computing.include?(n)  # cycle

    @xdr_fixed_size_computing.add(n)
    result = case defn
    when AST::Definitions::Enum
      4
    when AST::Definitions::Struct
      total = 0
      defn.members.each do |m|
        s = xdr_fixed_size_of_field(m.declaration.type)
        if s.nil?
          total = nil
          break
        end
        total += s
      end
      total
    when AST::Definitions::Union
      # discriminant is always 4 bytes (i32 or enum).
      # For the union to be fixed-size, all arms must have the same size.
      arm_sizes = []
      union_cases(defn) do |case_name, arm|
        if arm.void?
          arm_sizes << 0
        else
          s = xdr_fixed_size_of_field(arm.type)
          if s.nil?
            arm_sizes = nil
            break
          end
          arm_sizes << s
        end
      end
      if arm_sizes && arm_sizes.uniq.length == 1
        4 + arm_sizes.first  # discriminant + arm
      else
        nil
      end
    when AST::Definitions::Typedef
      if is_builtin_type(defn.type)
        xdr_fixed_size_of_type(defn.type)
      else
        xdr_fixed_size_of_field(defn.type)
      end
    else
      nil
    end
    @xdr_fixed_size_computing.delete(n)
    @xdr_fixed_size_cache[n] = result
    result
  end

  def is_fixed_size_defn(defn)
    !xdr_fixed_size_of_defn(defn).nil?
  end

  # ---------------------------------------------------------------------------
  # Skip code generation — emit Rust code that advances `pos` past one field.
  # `pos` is the name of a `u32` variable in scope.
  # ---------------------------------------------------------------------------

  # Returns Rust code (as a string) that, given a `pos` variable holding the
  # current offset, advances `pos` past one instance of the given field type.
  # For fixed-size fields this is a constant addition. For variable-size it
  # reads length prefixes.
  def skip_field_rust(type, pos)
    case type.sub_type
    when :simple
      skip_simple_rust(type, pos)
    when :optional
      inner_skip = skip_simple_rust(type, pos) # reuse inner logic
      # Option: 4-byte flag, then conditionally T.
      "{ #{pos} = #{pos}.checked_add(skip_option(buf, #{pos}, |buf, off| { let mut p = off; #{skip_simple_rust_inline(type, 'p')}; Ok(p.checked_sub(off).ok_or(Error::Invalid)?) })?).ok_or(Error::LengthExceedsMax)?; }"
    when :array
      elem_size = xdr_fixed_size_of_type(type)
      is_named, size = type.array_size
      size_val = is_named ? "#{name @top.find_definition(size)} as u32" : size.to_s
      if elem_size
        total = is_named ? nil : elem_size * size.to_i
        if total
          "{ #{pos} = #{pos}.checked_add(#{total}).ok_or(Error::LengthExceedsMax)?; }"
        else
          "{ #{pos} = #{pos}.checked_add((#{size_val}).checked_mul(#{elem_size}).ok_or(Error::LengthExceedsMax)?).ok_or(Error::LengthExceedsMax)?; }"
        end
      else
        # Variable-size elements
        "{ for _ in 0..#{size_val} { #{skip_simple_rust_inline(type, pos)} } }"
      end
    when :var_array
      base_ref = base_reference(type)
      max = type.decl.resolved_size || "u32::MAX"
      if base_ref == 'u8' || AST::Typespecs::String === type || AST::Typespecs::Opaque === type
        "{ #{pos} = #{pos}.checked_add(skip_var_opaque(buf, #{pos}, #{max})?).ok_or(Error::LengthExceedsMax)?; }"
      else
        elem_size = xdr_fixed_size_of_type(type)
        if elem_size
          "{ #{pos} = #{pos}.checked_add(skip_vec_fixed(buf, #{pos}, #{elem_size}, #{max})?).ok_or(Error::LengthExceedsMax)?; }"
        else
          "{ #{pos} = #{pos}.checked_add(skip_vec_var(buf, #{pos}, #{max}, |buf, off| { let mut p = off; #{skip_simple_rust_inline(type, 'p')}; Ok(p.checked_sub(off).ok_or(Error::Invalid)?) })?).ok_or(Error::LengthExceedsMax)?; }"
        end
      end
    else
      raise "Unknown sub_type: #{type.sub_type}"
    end
  end

  # Skip a simple (non-optional, non-array) type.
  def skip_simple_rust(type, pos)
    fs = xdr_fixed_size_of_type(type)
    if fs
      "{ #{pos} = #{pos}.checked_add(#{fs}).ok_or(Error::LengthExceedsMax)?; }"
    else
      skip_simple_rust_variable(type, pos)
    end
  end

  # Inline skip for simple types: produces statements (no braces wrapping)
  # suitable for embedding in closures. Mutates `pos` in place.
  def skip_simple_rust_inline(type, pos)
    fs = xdr_fixed_size_of_type(type)
    if fs
      "#{pos} = #{pos}.checked_add(#{fs}).ok_or(Error::LengthExceedsMax)?;"
    else
      skip_simple_rust_variable_inline(type, pos)
    end
  end

  # Skip a variable-size simple type. Generates Rust to call the appropriate
  # generated `skip_xdr_<type>` function.
  def skip_simple_rust_variable(type, pos)
    base_ref = base_reference(type)
    case type
    when AST::Typespecs::String
      max = type.decl.resolved_size || "u32::MAX"
      "{ #{pos} = #{pos}.checked_add(skip_var_opaque(buf, #{pos}, #{max})?).ok_or(Error::LengthExceedsMax)?; }"
    when AST::Typespecs::Opaque
      if !type.fixed?
        max = type.decl.resolved_size || "u32::MAX"
        "{ #{pos} = #{pos}.checked_add(skip_var_opaque(buf, #{pos}, #{max})?).ok_or(Error::LengthExceedsMax)?; }"
      else
        n = type.size.to_i
        total = n + ((4 - (n % 4)) % 4)
        "{ #{pos} = #{pos}.checked_add(#{total}).ok_or(Error::LengthExceedsMax)?; }"
      end
    else
      # Named type — call its generated validate/skip function.
      "{ let l = #{base_ref}::xdr_ref_validate(buf, #{pos}, &mut RefLimits::none())?; #{pos} = #{pos}.checked_add(l).ok_or(Error::LengthExceedsMax)?; }"
    end
  end

  def skip_simple_rust_variable_inline(type, pos)
    skip_simple_rust_variable(type, pos).sub(/^\{ /, '').sub(/ \}$/, '')
  end

  # ---------------------------------------------------------------------------
  # Validate code generation — emit Rust code that validates a type at `pos`
  # and returns the byte length consumed. Used in validate functions for
  # struct fields and union arms.
  # ---------------------------------------------------------------------------

  # Generate Rust code to validate a field at `pos`, advancing `pos` past it.
  # Uses `limits` (a RefLimits) for depth-limited validation.
  def validate_field_rust(parent_defn, type, pos, limits)
    case type.sub_type
    when :simple
      validate_simple_rust(parent_defn, type, pos, limits)
    when :optional
      inner_validate = validate_simple_rust_inner(parent_defn, type, pos, limits)
      "{ let flag = read_u32_at(buf, #{pos})?; match flag { 0 => { #{pos} = #{pos}.checked_add(4).ok_or(Error::LengthExceedsMax)?; } 1 => { #{pos} = #{pos}.checked_add(4).ok_or(Error::LengthExceedsMax)?; #{inner_validate}; } _ => return Err(Error::Invalid), } }"
    when :array
      elem_size = xdr_fixed_size_of_type(type)
      is_named, size = type.array_size
      size_val = is_named ? size.to_s : size.to_s
      if elem_size
        total = elem_size * size.to_i
        "{ check_bounds(buf, #{pos}, #{total})?; #{pos} = #{pos}.checked_add(#{total}).ok_or(Error::LengthExceedsMax)?; }"
      else
        inner_validate = validate_simple_rust_inner(parent_defn, type, pos, limits)
        "{ for _ in 0..#{size_val}u32 { #{inner_validate}; } }"
      end
    when :var_array
      base_ref = base_reference(type)
      max = type.decl.resolved_size || "u32::MAX"
      if base_ref == 'u8' || AST::Typespecs::String === type || AST::Typespecs::Opaque === type
        "{ #{pos} = #{pos}.checked_add(skip_var_opaque(buf, #{pos}, #{max})?).ok_or(Error::LengthExceedsMax)?; }"
      else
        elem_size = xdr_fixed_size_of_type(type)
        if elem_size
          "{ #{pos} = #{pos}.checked_add(skip_vec_fixed(buf, #{pos}, #{elem_size}, #{max})?).ok_or(Error::LengthExceedsMax)?; }"
        else
          inner_validate = validate_simple_rust_inner(parent_defn, type, pos, limits)
          "{ let count = read_u32_at(buf, #{pos})?; if count > #{max} { return Err(Error::LengthExceedsMax); } #{pos} = #{pos}.checked_add(4).ok_or(Error::LengthExceedsMax)?; for _ in 0..count { #{inner_validate}; } }"
        end
      end
    else
      raise "Unknown sub_type: #{type.sub_type}"
    end
  end

  # Validate a simple (non-array, non-optional) type. Advances `pos`.
  def validate_simple_rust(parent_defn, type, pos, limits)
    fs = xdr_fixed_size_of_type(type)
    if fs
      "{ check_bounds(buf, #{pos}, #{fs})?; #{pos} = #{pos}.checked_add(#{fs}).ok_or(Error::LengthExceedsMax)?; }"
    else
      validate_simple_rust_variable(parent_defn, type, pos, limits)
    end
  end

  # Validate a simple type at `pos_var`, advancing `pos_var` past it.
  # Returns the inline Rust expression (no braces wrapping).
  def validate_simple_rust_inner(parent_defn, type, pos_var, limits)
    fs = xdr_fixed_size_of_type(type)
    if fs
      "check_bounds(buf, #{pos_var}, #{fs})?; #{pos_var} = #{pos_var}.checked_add(#{fs}).ok_or(Error::LengthExceedsMax)?;"
    else
      case type
      when AST::Typespecs::String
        max = type.decl.resolved_size || "u32::MAX"
        "#{pos_var} = #{pos_var}.checked_add(skip_var_opaque(buf, #{pos_var}, #{max})?).ok_or(Error::LengthExceedsMax)?;"
      when AST::Typespecs::Opaque
        if !type.fixed?
          max = type.decl.resolved_size || "u32::MAX"
          "#{pos_var} = #{pos_var}.checked_add(skip_var_opaque(buf, #{pos_var}, #{max})?).ok_or(Error::LengthExceedsMax)?;"
        else
          n = type.size.to_i
          total = n + ((4 - (n % 4)) % 4)
          "check_bounds(buf, #{pos_var}, #{total})?; #{pos_var} = #{pos_var}.checked_add(#{total}).ok_or(Error::LengthExceedsMax)?;"
        end
      else
        base = base_reference(type)
        "{ let l = #{base}::xdr_ref_validate(buf, #{pos_var}, #{limits})?; #{pos_var} = #{pos_var}.checked_add(l).ok_or(Error::LengthExceedsMax)?; }"
      end
    end
  end

  def validate_simple_rust_variable(parent_defn, type, pos, limits)
    base = base_reference(type)
    case type
    when AST::Typespecs::String
      max = type.decl.resolved_size || "u32::MAX"
      "{ #{pos} = #{pos}.checked_add(skip_var_opaque(buf, #{pos}, #{max})?).ok_or(Error::LengthExceedsMax)?; }"
    when AST::Typespecs::Opaque
      if !type.fixed?
        max = type.decl.resolved_size || "u32::MAX"
        "{ #{pos} = #{pos}.checked_add(skip_var_opaque(buf, #{pos}, #{max})?).ok_or(Error::LengthExceedsMax)?; }"
      else
        n = type.size.to_i
        total = n + ((4 - (n % 4)) % 4)
        "{ check_bounds(buf, #{pos}, #{total})?; #{pos} = #{pos}.checked_add(#{total}).ok_or(Error::LengthExceedsMax)?; }"
      end
    else
      "{ let l = #{base}::xdr_ref_validate(buf, #{pos}, #{limits})?; #{pos} = #{pos}.checked_add(l).ok_or(Error::LengthExceedsMax)?; }"
    end
  end

  # ---------------------------------------------------------------------------
  # Ref rendering — zero-copy reference types
  # ---------------------------------------------------------------------------

  def render_ref_top_matter(out)
    out.puts <<-EOS.strip_heredoc
      // Module #{@namespace}_refs is generated from:
      //  #{@output.relative_source_paths.join("\n//  ")}
      //
      // Zero-copy buffer reference types for XDR.

      #![allow(clippy::missing_errors_doc, clippy::unreadable_literal, clippy::too_many_lines, dead_code)]
    EOS
    out.break
    out.puts "use super::generated::*;"
    out.puts "use super::generated::{read_u32_at, read_i32_at, read_u64_at, read_i64_at, read_bool_at, read_bytes_at};"
    out.puts "use super::generated::{check_bounds, check_padding, pad_len_ref, var_opaque_xdr_len};"
    out.puts "use super::generated::{skip_var_opaque, skip_option, skip_vec_fixed, skip_vec_var};"
    out.puts "use super::generated::{validate_i32, validate_u32, validate_i64, validate_u64, validate_bool, validate_void};"
    out.puts "use super::generated::{validate_fixed_opaque, validate_option, validate_box};"
    out.puts "use super::generated::{validate_vec, validate_vec_fixed, validate_vec_u8};"
    out.puts "use super::generated::{validate_bytes, validate_string};"
    out.puts "use super::generated::{validate_fixed_array, validate_fixed_array_var};"
    out.puts "use super::generated::{cmp_i32, cmp_u32, cmp_i64, cmp_u64, cmp_bool, cmp_bytes};"
    out.puts "use super::generated::{XdrRef, RefLimits, Error};"
    out.puts "#[allow(unused_imports)]"
    out.puts "use super::generated::{VecM, BytesM, StringM, Limits, Limited, ReadXdr, WriteXdr};"
    out.puts "use core::cmp::Ordering;"
    out.break
  end

  def render_ref_lib(out)
    # No additional header needed; all infrastructure is in header_refs.rs
    # which is included in the main generated.rs.
  end

  def render_definitions_ref(out, node)
    node.definitions.each{|n| render_definition_ref out, n }
    node.namespaces.each{|n| render_definitions_ref out, n }
  end

  def render_definition_ref(out, defn)
    if @already_rendered_ref.include? name(defn)
      return
    end

    render_nested_definitions_ref(out, defn)

    @already_rendered_ref << name(defn)

    case defn
    when AST::Definitions::Struct ;
      render_struct_ref out, defn
    when AST::Definitions::Enum ;
      render_enum_ref out, defn
    when AST::Definitions::Union ;
      render_union_ref out, defn
    when AST::Definitions::Typedef ;
      render_typedef_ref out, defn
    when AST::Definitions::Const ;
      # Constants don't need ref types
    end
  end

  def render_nested_definitions_ref(out, defn)
    return unless defn.respond_to? :nested_definitions
    defn.nested_definitions.each{|ndefn| render_definition_ref out, ndefn}
  end

  # ---------------------------------------------------------------------------
  # Enum ref
  # ---------------------------------------------------------------------------

  def render_enum_ref(out, enum)
    n = name(enum)
    out.puts "/// Ref type alias for [`#{n}`]."
    out.puts "pub type #{n}Ref = XdrRef<#{n}>;"
    out.puts ""

    out.puts <<-EOS.strip_heredoc
    impl #{n} {
        /// Validate the XDR encoding of #{n} at `offset` in `buf`.
        /// Returns the XDR byte length (always 4).
        pub fn xdr_ref_validate(buf: &[u8], offset: u32, _limits: &mut RefLimits) -> Result<u32, Error> {
            check_bounds(buf, offset, 4)?;
            let v = read_i32_at(buf, offset)?;
            let _: #{n} = v.try_into()?;
            Ok(4)
        }

        /// Validate and construct an `XdrRef<#{n}>` at `offset`.
        pub fn xdr_ref_from(buf: &[u8], offset: u32, limits: &mut RefLimits) -> Result<XdrRef<#{n}>, Error> {
            let len = Self::xdr_ref_validate(buf, offset, limits)?;
            Ok(XdrRef::new(offset, len))
        }
    }

    impl XdrRef<#{n}> {
        pub const XDR_FIXED_SIZE: u32 = 4;

        /// Read the enum value from the buffer.
        #[inline]
        pub fn get(&self, buf: &[u8]) -> Result<#{n}, Error> {
            let v = read_i32_at(buf, self.offset())?;
            v.try_into()
        }

        /// Materialize the enum value.
        #[inline]
        pub fn materialize(&self, buf: &[u8]) -> Result<#{n}, Error> {
            self.get(buf)
        }

        /// Compare two enum refs by their discriminant value.
        #[inline]
        pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
            cmp_i32(buf, self.offset(), other_buf, other.offset())
        }
    }
    EOS
    out.break
  end

  # ---------------------------------------------------------------------------
  # Struct ref
  # ---------------------------------------------------------------------------

  def render_struct_ref(out, struct)
    n = name(struct)
    fixed_size = xdr_fixed_size_of_defn(struct)

    out.puts "/// Ref type alias for [`#{n}`]."
    out.puts "pub type #{n}Ref = XdrRef<#{n}>;"
    out.puts ""

    # --- validate ---
    out.puts "impl #{n} {"
    out.puts "    /// Validate the XDR encoding of #{n} at `offset` in `buf`."
    out.puts "    /// Returns the total XDR byte length."
    out.puts "    pub fn xdr_ref_validate(buf: &[u8], offset: u32, limits: &mut RefLimits) -> Result<u32, Error> {"
    out.puts "        limits.with_limited_depth(|limits| {"
    out.puts "            let mut pos = offset;"
    # Batch consecutive fixed-size fields into one bounds check + add.
    pending_fixed = 0
    struct.members.each do |m|
      fs = xdr_fixed_size_of_field(m.declaration.type)
      if fs
        pending_fixed += fs
      else
        # Flush any accumulated fixed-size bytes first.
        if pending_fixed > 0
          out.puts "            check_bounds(buf, pos, #{pending_fixed})?; pos = pos.checked_add(#{pending_fixed}).ok_or(Error::LengthExceedsMax)?;"
          pending_fixed = 0
        end
        out.puts "            #{validate_field_rust(struct, m.declaration.type, 'pos', 'limits')}"
      end
    end
    if pending_fixed > 0
      out.puts "            check_bounds(buf, pos, #{pending_fixed})?; pos = pos.checked_add(#{pending_fixed}).ok_or(Error::LengthExceedsMax)?;"
    end
    out.puts "            Ok(pos.checked_sub(offset).ok_or(Error::Invalid)?)"
    out.puts "        })"
    out.puts "    }"
    out.puts ""
    out.puts "    /// Validate and construct an `XdrRef<#{n}>` at `offset`."
    out.puts "    pub fn xdr_ref_from(buf: &[u8], offset: u32, limits: &mut RefLimits) -> Result<XdrRef<#{n}>, Error> {"
    out.puts "        let len = Self::xdr_ref_validate(buf, offset, limits)?;"
    out.puts "        Ok(XdrRef::new(offset, len))"
    out.puts "    }"
    out.puts "}"
    out.puts ""

    # --- accessors ---
    out.puts "impl XdrRef<#{n}> {"
    if fixed_size
      out.puts "    pub const XDR_FIXED_SIZE: u32 = #{fixed_size};"
      out.puts ""
    end

    # Per-field accessors
    cumulative_offset = 0
    all_fixed_so_far = true
    struct.members.each_with_index do |m, idx|
      fname = field_name(m)
      field_type = m.declaration.type
      field_ref_type = ref_type_for_field(struct, field_type)
      field_fs = xdr_fixed_size_of_field(field_type)

      if all_fixed_so_far
        # This field is at a known constant offset from the struct start.
        out.puts "    /// Access field `#{fname}` at a #{field_fs ? 'fixed' : 'variable'} offset."
        out.puts "    #[inline]"
        out.puts "    pub fn #{fname}(&self, buf: &[u8]) -> Result<#{field_ref_type}, Error> {"
        out.puts "        let field_off = self.offset().checked_add(#{cumulative_offset}).ok_or(Error::LengthExceedsMax)?;"
        out.puts render_field_accessor_body(struct, field_type, 'field_off')
        out.puts "    }"
        out.puts ""
        if field_fs
          cumulative_offset += field_fs
        else
          all_fixed_so_far = false
        end
      else
        # This field is at a variable offset. Scan from struct start.
        out.puts "    /// Access field `#{fname}` (variable offset — scans preceding fields)."
        out.puts "    pub fn #{fname}(&self, buf: &[u8]) -> Result<#{field_ref_type}, Error> {"
        out.puts "        let mut pos = self.offset();"
        # Skip all preceding fields. Batch consecutive fixed-size skips.
        skip_pending = 0
        struct.members[0...idx].each do |prev_m|
          prev_fs = xdr_fixed_size_of_field(prev_m.declaration.type)
          if prev_fs
            skip_pending += prev_fs
          else
            if skip_pending > 0
              out.puts "        pos = pos.checked_add(#{skip_pending}).ok_or(Error::LengthExceedsMax)?;"
              skip_pending = 0
            end
            out.puts "        #{skip_field_rust(prev_m.declaration.type, 'pos')}"
          end
        end
        if skip_pending > 0
          out.puts "        pos = pos.checked_add(#{skip_pending}).ok_or(Error::LengthExceedsMax)?;"
        end
        out.puts "        let field_off = pos;"
        out.puts render_field_accessor_body(struct, field_type, 'field_off')
        out.puts "    }"
        out.puts ""
      end
    end

    # --- materialize ---
    out.puts "    /// Materialize the full owned [`#{n}`] from the buffer."
    out.puts "    #[cfg(feature = \"std\")]"
    out.puts "    pub fn materialize(&self, buf: &[u8]) -> Result<#{n}, Error> {"
    out.puts "        let s = self.as_slice(buf)?;"
    out.puts "        let mut cursor = Limited::new(std::io::Cursor::new(s), Limits::len(s.len()));"
    out.puts "        #{n}::read_xdr(&mut cursor)"
    out.puts "    }"
    out.puts ""

    # --- cmp_in ---
    out.puts "    /// Field-by-field comparison matching derived `Ord`."
    out.puts "    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {"
    struct.members.each_with_index do |m, idx|
      fname = field_name(m)
      out.puts "        let ord = self.#{fname}(buf)?.cmp_in(buf, &other.#{fname}(other_buf)?, other_buf)?;"
      out.puts "        if ord != Ordering::Equal { return Ok(ord); }"
    end
    out.puts "        Ok(Ordering::Equal)"
    out.puts "    }"

    out.puts "}"
    out.break
  end

  # Returns the Rust ref type for a given field type (the return type of an accessor).
  def ref_type_for_field(parent, type)
    base = base_reference(type)
    parent_name = name(parent) if parent
    cyclic = parent_name && is_type_in_type_field_types(base, parent_name)

    case type.sub_type
    when :simple
      if cyclic
        "XdrRef<Box<#{base}>>"
      else
        "XdrRef<#{base}>"
      end
    when :optional
      if cyclic
        "XdrRef<Option<Box<#{base}>>>"
      else
        "XdrRef<Option<#{base}>>"
      end
    when :array
      if AST::Typespecs::Opaque === type && type.fixed?
        "XdrRef<[u8; #{type.size}]>"
      else
        is_named, size = type.array_size
        size = name @top.find_definition(size) if is_named
        "XdrRef<[#{base}; #{size}]>"
      end
    when :var_array
      if AST::Typespecs::String === type
        max = type.decl.resolved_size
        max ? "XdrRef<StringM<#{max}>>" : "XdrRef<StringM>"
      elsif AST::Typespecs::Opaque === type
        max = type.decl.resolved_size
        max ? "XdrRef<BytesM<#{max}>>" : "XdrRef<BytesM>"
      else
        max = type.decl.resolved_size
        max ? "XdrRef<VecM<#{base}, #{max}>>" : "XdrRef<VecM<#{base}>>"
      end
    else
      raise "Unknown sub_type: #{type.sub_type}"
    end
  end

  # Generate the body of a field accessor. Returns an XdrRef for the field.
  def render_field_accessor_body(parent, type, off_var)
    base = base_reference(type)
    parent_name = name(parent) if parent
    cyclic = parent_name && is_type_in_type_field_types(base, parent_name)

    case type.sub_type
    when :simple
      fs = xdr_fixed_size_of_type(type)
      if fs
        "        Ok(XdrRef::new(#{off_var}, #{fs}))"
      else
        # Check for parameterized builtins (StringM, BytesM) that don't have
        # xdr_ref_validate as an inherent method.
        case type
        when AST::Typespecs::String
          max = type.decl.resolved_size || "u32::MAX"
          "        let (_, total) = var_opaque_xdr_len(buf, #{off_var}, #{max})?;\n        Ok(XdrRef::new(#{off_var}, total))"
        when AST::Typespecs::Opaque
          if !type.fixed?
            max = type.decl.resolved_size || "u32::MAX"
            "        let (_, total) = var_opaque_xdr_len(buf, #{off_var}, #{max})?;\n        Ok(XdrRef::new(#{off_var}, total))"
          else
            n = type.size.to_i
            total = n + ((4 - (n % 4)) % 4)
            "        Ok(XdrRef::new(#{off_var}, #{total}))"
          end
        else
          # Variable-size named type — compute length.
          "        let l = #{base}::xdr_ref_validate(buf, #{off_var}, &mut RefLimits::none())?;\n        Ok(XdrRef::new(#{off_var}, l))"
        end
      end
    when :optional
      inner_size = xdr_fixed_size_of_type(type)
      if inner_size
        lines = "        let flag = read_u32_at(buf, #{off_var})?;\n"
        lines += "        match flag {\n"
        lines += "            0 => Ok(XdrRef::new(#{off_var}, 4)),\n"
        lines += "            1 => Ok(XdrRef::new(#{off_var}, #{4 + inner_size})),\n"
        lines += "            _ => Err(Error::Invalid),\n"
        lines += "        }"
        lines
      else
        lines = "        let flag = read_u32_at(buf, #{off_var})?;\n"
        lines += "        match flag {\n"
        lines += "            0 => Ok(XdrRef::new(#{off_var}, 4)),\n"
        lines += "            1 => {\n"
        lines += "                let inner_off = #{off_var}.checked_add(4).ok_or(Error::LengthExceedsMax)?;\n"
        lines += "                let inner_len = #{base}::xdr_ref_validate(buf, inner_off, &mut RefLimits::none())?;\n"
        lines += "                Ok(XdrRef::new(#{off_var}, 4u32.checked_add(inner_len).ok_or(Error::LengthExceedsMax)?))\n"
        lines += "            }\n"
        lines += "            _ => Err(Error::Invalid),\n"
        lines += "        }"
        lines
      end
    when :array
      if AST::Typespecs::Opaque === type && type.fixed?
        n = type.size.to_i
        total = n + ((4 - (n % 4)) % 4)
        "        Ok(XdrRef::new(#{off_var}, #{total}))"
      else
        elem_size = xdr_fixed_size_of_type(type)
        is_named, size = type.array_size
        size_int = size.to_i
        if elem_size
          total = elem_size * size_int
          "        Ok(XdrRef::new(#{off_var}, #{total}))"
        else
          # Variable-size elements — scan
          lines = "        let mut p = #{off_var};\n"
          lines += "        for _ in 0..#{size_int}u32 {\n"
          lines += "            let l = #{base}::xdr_ref_validate(buf, p, &mut RefLimits::none())?;\n"
          lines += "            p = p.checked_add(l).ok_or(Error::LengthExceedsMax)?;\n"
          lines += "        }\n"
          lines += "        Ok(XdrRef::new(#{off_var}, p.checked_sub(#{off_var}).ok_or(Error::Invalid)?))"
          lines
        end
      end
    when :var_array
      if AST::Typespecs::String === type || AST::Typespecs::Opaque === type
        max = type.decl.resolved_size || "u32::MAX"
        lines = "        let (_, total) = var_opaque_xdr_len(buf, #{off_var}, #{max})?;\n"
        lines += "        Ok(XdrRef::new(#{off_var}, total))"
        lines
      else
        max = type.decl.resolved_size || "u32::MAX"
        elem_size = xdr_fixed_size_of_type(type)
        if elem_size
          lines = "        let total = skip_vec_fixed(buf, #{off_var}, #{elem_size}, #{max})?;\n"
          lines += "        Ok(XdrRef::new(#{off_var}, total))"
          lines
        else
          lines = "        let total = skip_vec_var(buf, #{off_var}, #{max}, |buf, off| {\n"
          lines += "            let mut p = off;\n"
          lines += "            let l = #{base}::xdr_ref_validate(buf, p, &mut RefLimits::none())?;\n"
          lines += "            p = p.checked_add(l).ok_or(Error::LengthExceedsMax)?;\n"
          lines += "            Ok(p.checked_sub(off).ok_or(Error::Invalid)?)\n"
          lines += "        })?;\n"
          lines += "        Ok(XdrRef::new(#{off_var}, total))"
          lines
        end
      end
    else
      raise "Unknown sub_type: #{type.sub_type}"
    end
  end

  # ---------------------------------------------------------------------------
  # Union ref
  # ---------------------------------------------------------------------------

  def render_union_ref(out, union)
    n = name(union)
    discriminant_type = reference(nil, union.discriminant.type)
    discriminant_type_builtin = is_builtin_type(union.discriminant.type) || (is_builtin_type(union.discriminant.type.resolved_type.type) if union.discriminant.type.respond_to?(:resolved_type) && AST::Definitions::Typedef === union.discriminant.type.resolved_type)
    fixed_size = xdr_fixed_size_of_defn(union)

    # Determine the correct reader for the discriminant based on its type.
    disc_unsigned = (AST::Typespecs::UnsignedInt === union.discriminant.type) ||
      (union.discriminant.type.respond_to?(:resolved_type) &&
       AST::Definitions::Typedef === union.discriminant.type.resolved_type &&
       AST::Typespecs::UnsignedInt === union.discriminant.type.resolved_type.type)
    disc_read_fn = disc_unsigned ? "read_u32_at" : "read_i32_at"

    out.puts "/// Ref type alias for [`#{n}`]."
    out.puts "pub type #{n}Ref = XdrRef<#{n}>;"
    out.puts ""

    # --- validate ---
    out.puts "impl #{n} {"
    out.puts "    /// Validate the XDR encoding of #{n} at `offset` in `buf`."
    out.puts "    /// Returns the total XDR byte length."
    out.puts "    pub fn xdr_ref_validate(buf: &[u8], offset: u32, limits: &mut RefLimits) -> Result<u32, Error> {"
    out.puts "        limits.with_limited_depth(|limits| {"
    out.puts "            let dv_ref = validate_i32(buf, offset)?;"
    out.puts "            let dv = #{disc_read_fn}(buf, offset)?;"
    out.puts "            let mut pos = offset.checked_add(4).ok_or(Error::LengthExceedsMax)?;"

    # Determine discriminant matching pattern
    if discriminant_type_builtin
      out.puts "            #[allow(clippy::match_same_arms)]"
      out.puts "            match dv {"
      union_cases(union) do |case_name, arm, value|
        if arm.void?
          out.puts "                #{value} => {},"
        else
          out.puts "                #{value} => { #{validate_field_rust(union, arm.type, 'pos', 'limits')} },"
        end
      end
    else
      out.puts "            let disc: #{discriminant_type} = dv.try_into()?;"
      out.puts "            #[allow(clippy::match_same_arms)]"
      out.puts "            match disc {"
      union_cases(union) do |case_name, arm, value|
        if arm.void?
          out.puts "                #{discriminant_type}::#{case_name} => {},"
        else
          out.puts "                #{discriminant_type}::#{case_name} => { #{validate_field_rust(union, arm.type, 'pos', 'limits')} },"
        end
      end
    end
    out.puts "                #[allow(unreachable_patterns)]"
    out.puts "                _ => return Err(Error::Invalid),"
    out.puts "            }"
    out.puts "            Ok(pos.checked_sub(offset).ok_or(Error::Invalid)?)"
    out.puts "        })"
    out.puts "    }"
    out.puts ""
    out.puts "    /// Validate and construct an `XdrRef<#{n}>` at `offset`."
    out.puts "    pub fn xdr_ref_from(buf: &[u8], offset: u32, limits: &mut RefLimits) -> Result<XdrRef<#{n}>, Error> {"
    out.puts "        let len = Self::xdr_ref_validate(buf, offset, limits)?;"
    out.puts "        Ok(XdrRef::new(offset, len))"
    out.puts "    }"
    out.puts "}"
    out.puts ""

    # --- accessors ---
    out.puts "impl XdrRef<#{n}> {"
    if fixed_size
      out.puts "    pub const XDR_FIXED_SIZE: u32 = #{fixed_size};"
      out.puts ""
    end

    # Discriminant accessor
    out.puts "    /// Read the discriminant."
    out.puts "    #[inline]"
    out.puts "    pub fn discriminant(&self, buf: &[u8]) -> Result<#{discriminant_type}, Error> {"
    if discriminant_type_builtin
      out.puts "        #{disc_read_fn}(buf, self.offset())"
    else
      out.puts "        let v = #{disc_read_fn}(buf, self.offset())?;"
      out.puts "        v.try_into()"
    end
    out.puts "    }"
    out.puts ""

    # Per-arm accessors
    union_cases(union) do |case_name, arm, value|
      arm_fname = case_name.underscore
      unless arm.void?
        arm_ref_type = ref_type_for_field(union, arm.type)
        out.puts "    /// Access the `#{case_name}` arm. Returns `Ok(None)` if a different arm is active."
        out.puts "    pub fn as_#{arm_fname}(&self, buf: &[u8]) -> Result<Option<#{arm_ref_type}>, Error> {"
        if discriminant_type_builtin
          out.puts "        let dv = #{disc_read_fn}(buf, self.offset())?;"
          out.puts "        if dv != #{value} { return Ok(None); }"
        else
          out.puts "        let disc = self.discriminant(buf)?;"
          out.puts "        if disc != #{discriminant_type}::#{case_name} { return Ok(None); }"
        end
        out.puts "        let arm_off = self.offset().checked_add(4).ok_or(Error::LengthExceedsMax)?;"
        out.puts render_field_accessor_body(union, arm.type, 'arm_off').lines.map { |l| "    #{l}" }.join
        out.puts "            .map(Some)"
        out.puts "    }"
        out.puts ""
      end
    end

    # materialize
    out.puts "    /// Materialize the full owned [`#{n}`] from the buffer."
    out.puts "    #[cfg(feature = \"std\")]"
    out.puts "    pub fn materialize(&self, buf: &[u8]) -> Result<#{n}, Error> {"
    out.puts "        let s = self.as_slice(buf)?;"
    out.puts "        let mut cursor = Limited::new(std::io::Cursor::new(s), Limits::len(s.len()));"
    out.puts "        #{n}::read_xdr(&mut cursor)"
    out.puts "    }"
    out.puts ""

    # cmp_in — compare discriminant first, then arm
    out.puts "    /// Compare two union refs: discriminant first, then arm data."
    out.puts "    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {"
    out.puts "        let d1 = #{disc_read_fn}(buf, self.offset())?;"
    out.puts "        let d2 = #{disc_read_fn}(other_buf, other.offset())?;"
    out.puts "        let disc_ord = d1.cmp(&d2);"
    out.puts "        if disc_ord != Ordering::Equal { return Ok(disc_ord); }"
    out.puts "        // Same discriminant — compare arm data bytes."
    out.puts "        let arm1_off = self.offset().checked_add(4).ok_or(Error::LengthExceedsMax)?;"
    out.puts "        let arm1_len = self.len().checked_sub(4).ok_or(Error::Invalid)?;"
    out.puts "        let arm2_off = other.offset().checked_add(4).ok_or(Error::LengthExceedsMax)?;"
    out.puts "        let arm2_len = other.len().checked_sub(4).ok_or(Error::Invalid)?;"
    out.puts "        // For union arms with the same discriminant, the XDR bytes are the same type."
    out.puts "        // Byte comparison is correct for unsigned/opaque. For signed ints or complex"
    out.puts "        // types we'd need field-by-field comparison. Use byte comparison as a"
    out.puts "        // reasonable default matching the existing derived Ord on the owned type."
    out.puts "        cmp_bytes(buf, arm1_off, arm1_len, other_buf, arm2_off, arm2_len)"
    out.puts "    }"

    out.puts "}"
    out.break
  end

  # ---------------------------------------------------------------------------
  # Typedef ref
  # ---------------------------------------------------------------------------

  def render_typedef_ref(out, typedef)
    n = name(typedef)

    if is_builtin_type(typedef.type)
      # Builtin type alias — the ref is just XdrRef<base_type>.
      # No separate type needed (users use XdrRef<i32> etc directly).
      return
    end

    out.puts "/// Ref type alias for [`#{n}`]."
    out.puts "pub type #{n}Ref = XdrRef<#{n}>;"
    out.puts ""

    inner_ref = base_reference(typedef.type)

    # --- validate ---
    out.puts "impl #{n} {"
    out.puts "    /// Validate the XDR encoding of #{n} at `offset` in `buf`."
    out.puts "    /// Returns the total XDR byte length."
    out.puts "    pub fn xdr_ref_validate(buf: &[u8], offset: u32, limits: &mut RefLimits) -> Result<u32, Error> {"

    if is_fixed_array_opaque(typedef.type)
      size = typedef.type.size.to_i
      total = size + ((4 - (size % 4)) % 4)
      out.puts "        check_bounds(buf, offset, #{total})?;"
      out.puts "        check_padding(buf, offset.checked_add(#{size}).ok_or(Error::LengthExceedsMax)?, #{(4 - (size % 4)) % 4})?;"
      out.puts "        Ok(#{total})"
    elsif is_var_array_type(typedef.type)
      if AST::Typespecs::String === typedef.type
        max = typedef.type.decl.resolved_size || "u32::MAX"
        out.puts "        let (_, total) = var_opaque_xdr_len(buf, offset, #{max})?;"
        out.puts "        Ok(total)"
      elsif AST::Typespecs::Opaque === typedef.type
        max = typedef.type.decl.resolved_size || "u32::MAX"
        out.puts "        let (_, total) = var_opaque_xdr_len(buf, offset, #{max})?;"
        out.puts "        Ok(total)"
      else
        # VecM<T, MAX>
        elem_type_base = base_reference(typedef.type)
        max = typedef.type.decl.resolved_size || "u32::MAX"
        elem_size = xdr_fixed_size_of_type(typedef.type)
        if elem_size
          out.puts "        let r = skip_vec_fixed(buf, offset, #{elem_size}, #{max})?;"
          out.puts "        Ok(r)"
        else
          out.puts "        let count = read_u32_at(buf, offset)?;"
          out.puts "        if count > #{max} { return Err(Error::LengthExceedsMax); }"
          out.puts "        let mut pos = offset.checked_add(4).ok_or(Error::LengthExceedsMax)?;"
          out.puts "        for _ in 0..count {"
          # element_type_for_vec gives us the element type name
          elem_name = element_type_for_vec(typedef.type)
          if ['u8', 'i32', 'u32', 'i64', 'u64', 'bool'].include?(elem_name)
            fs = case elem_name
              when 'u8' then nil # shouldn't happen, handled by opaque
              when 'i32', 'u32', 'bool' then 4
              when 'i64', 'u64' then 8
            end
            if fs
              out.puts "            pos = pos.checked_add(#{fs}).ok_or(Error::LengthExceedsMax)?;"
            end
          else
            out.puts "            let l = #{elem_name}::xdr_ref_validate(buf, pos, limits)?;"
            out.puts "            pos = pos.checked_add(l).ok_or(Error::LengthExceedsMax)?;"
          end
          out.puts "        }"
          out.puts "        Ok(pos.checked_sub(offset).ok_or(Error::Invalid)?)"
        end
      end
    elsif is_fixed_array_type(typedef.type)
      # Fixed array of non-opaque (e.g., [SomeType; N])
      elem_size = xdr_fixed_size_of_type(typedef.type)
      is_named, size = typedef.type.array_size
      size_int = size.to_i
      if elem_size
        total = elem_size * size_int
        out.puts "        check_bounds(buf, offset, #{total})?;"
        out.puts "        Ok(#{total})"
      else
        elem_name = base_reference(typedef.type)
        out.puts "        let mut pos = offset;"
        out.puts "        for _ in 0..#{size_int}u32 {"
        out.puts "            let l = #{elem_name}::xdr_ref_validate(buf, pos, limits)?;"
        out.puts "            pos = pos.checked_add(l).ok_or(Error::LengthExceedsMax)?;"
        out.puts "        }"
        out.puts "        Ok(pos.checked_sub(offset).ok_or(Error::Invalid)?)"
      end
    else
      # Simple named type
      fs = xdr_fixed_size_of_type(typedef.type)
      if fs
        out.puts "        check_bounds(buf, offset, #{fs})?;"
        out.puts "        Ok(#{fs})"
      else
        out.puts "        #{inner_ref}::xdr_ref_validate(buf, offset, limits)"
      end
    end

    out.puts "    }"
    out.puts ""
    out.puts "    /// Validate and construct an `XdrRef<#{n}>` at `offset`."
    out.puts "    pub fn xdr_ref_from(buf: &[u8], offset: u32, limits: &mut RefLimits) -> Result<XdrRef<#{n}>, Error> {"
    out.puts "        let len = Self::xdr_ref_validate(buf, offset, limits)?;"
    out.puts "        Ok(XdrRef::new(offset, len))"
    out.puts "    }"
    out.puts "}"
    out.puts ""

    # --- accessors ---
    out.puts "impl XdrRef<#{n}> {"
    fixed_size = xdr_fixed_size_of_defn(typedef)
    if fixed_size
      out.puts "    pub const XDR_FIXED_SIZE: u32 = #{fixed_size};"
      out.puts ""
    end

    # Materialize
    out.puts "    /// Materialize the full owned [`#{n}`] from the buffer."
    out.puts "    #[cfg(feature = \"std\")]"
    out.puts "    pub fn materialize(&self, buf: &[u8]) -> Result<#{n}, Error> {"
    out.puts "        let s = self.as_slice(buf)?;"
    out.puts "        let mut cursor = Limited::new(std::io::Cursor::new(s), Limits::len(s.len()));"
    out.puts "        #{n}::read_xdr(&mut cursor)"
    out.puts "    }"
    out.puts ""

    # cmp_in — delegate to byte comparison (canonical XDR)
    out.puts "    /// Compare two typedef refs."
    out.puts "    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {"
    out.puts "        // Typedef wraps another type; byte comparison is correct for canonical XDR."
    out.puts "        let a = self.as_slice(buf)?;"
    out.puts "        let b = other.as_slice(other_buf)?;"
    out.puts "        Ok(a.cmp(b))"
    out.puts "    }"

    out.puts "}"
    out.break
  end

  # ---------------------------------------------------------------------------
  # Enum of all types (TypeRef, etc.)
  # ---------------------------------------------------------------------------

  def render_enum_of_all_types_ref(out, types)
    # No trait needed. Every generated type has an inherent `cmp_in` method,
    # and struct `cmp_in` calls each field's `cmp_in` directly.
  end

end
