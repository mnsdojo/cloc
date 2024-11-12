pub struct Constants;

 impl Constants {
    pub const COMMENT_PREFIXES: [&'static str; 3] = ["//", "#", ";"];

    pub const LANGUAGE_MAP: &[(&'static str, &'static str)] = &[
        // General-purpose languages
        ("rs", "Rust"),
        ("cpp", "C++"),
        ("h", "C++"),
        ("hpp", "C++"),
        ("c", "C"),
        ("cs", "C#"),
        ("py", "Python"),
        ("go", "Go"),
        ("java", "Java"),
        ("rb", "Ruby"),
        // Web development languages
        ("js", "JavaScript"),
        ("jsx", "JavaScript (React)"),
        ("ts", "TypeScript"),
        ("tsx", "TypeScript (React)"),
        ("html", "HTML"),
        ("css", "CSS"),
        ("scss", "SCSS"),
        ("sass", "SASS"),
        // Shell and scripting languages
        ("sh", "Shell"),
        ("bash", "Bash"),
        ("zsh", "Zsh"),
        ("ps1", "PowerShell"),
        ("bat", "Batch"),
        // Data and configuration formats
        ("json", "JSON"),
        ("yml", "YAML"),
        ("yaml", "YAML"),
        ("xml", "XML"),
        ("toml", "TOML"),
        ("ini", "INI"),
        // Database and query languages
        ("sql", "SQL"),
        ("psql", "PostgreSQL"),
        ("mysql", "MySQL"),
        // Additional file types
        ("md", "Markdown"),
        ("r", "R"),
        ("swift", "Swift"),
        ("kt", "Kotlin"),
        ("dart", "Dart"),
        ("php", "PHP"),
        ("pl", "Perl"),
        ("lua", "Lua"),
        ("m", "Objective-C"),
        ("scala", "Scala"),
        ("clj", "Clojure"),
        ("jl", "Julia"),
        ("rs", "Rust"),
    ];
}
