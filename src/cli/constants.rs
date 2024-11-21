pub struct Constants;

impl Constants {
    /// Language-specific comment markers including single-line and multi-line comments
    pub const COMMENT_MARKERS: &[(&'static str, &[&'static str])] = &[
        ("Rust", &["//", "/*", "*/"]),
        ("C", &["//", "/*", "*/"]),
        ("C++", &["//", "/*", "*/"]),
        ("Python", &["#", "'''", "\"\"\""]),
        ("JavaScript", &["//", "/*", "*/"]),
        ("TypeScript", &["//", "/*", "*/"]),
        ("HTML", &["<!--", "-->"]),
        ("CSS", &["/*", "*/"]),
        ("SQL", &["--", "/*", "*/"]),
        ("Ruby", &["#", "=begin", "=end"]),
        ("Shell", &["#"]),
        ("PHP", &["//", "#", "/*", "*/"]),
        ("Perl", &["#", "=pod", "=cut"]),
        ("Lua", &["--", "--[[", "]]"]),
    ];

    /// Maps file extensions to their corresponding programming languages
    pub const LANGUAGE_MAP: &[(&'static str, &'static str)] = &[
        // Systems Programming
        ("rs", "Rust"),
        ("c", "C"),
        ("h", "C"),
        ("cpp", "C++"),
        ("hpp", "C++"),
        ("cc", "C++"),
        ("cxx", "C++"),
        ("hxx", "C++"),
        // Managed Languages
        ("cs", "C#"),
        ("java", "Java"),
        ("class", "Java"),
        ("scala", "Scala"),
        ("kt", "Kotlin"),
        ("kts", "Kotlin"),
        ("swift", "Swift"),
        ("go", "Go"),
        // Scripting Languages
        ("py", "Python"),
        ("pyc", "Python"),
        ("rb", "Ruby"),
        ("php", "PHP"),
        ("pl", "Perl"),
        ("lua", "Lua"),
        ("tcl", "Tcl"),
        // Web Development
        ("js", "JavaScript"),
        ("jsx", "JavaScript (React)"),
        ("ts", "TypeScript"),
        ("tsx", "TypeScript (React)"),
        ("html", "HTML"),
        ("htm", "HTML"),
        ("css", "CSS"),
        ("scss", "SCSS"),
        ("sass", "SASS"),
        ("less", "LESS"),
        ("vue", "Vue"),
        ("svelte", "Svelte"),
        // Shell and Scripts
        ("sh", "Shell"),
        ("bash", "Bash"),
        ("zsh", "Zsh"),
        ("fish", "Fish"),
        ("ps1", "PowerShell"),
        ("psm1", "PowerShell"),
        ("bat", "Batch"),
        ("cmd", "Batch"),
        // Data and Config
        ("json", "JSON"),
        ("yml", "YAML"),
        ("yaml", "YAML"),
        ("xml", "XML"),
        ("toml", "TOML"),
        ("ini", "INI"),
        ("conf", "Config"),
        ("cfg", "Config"),
        // Database
        ("sql", "SQL"),
        ("psql", "PostgreSQL"),
        ("mysql", "MySQL"),
        ("pgsql", "PostgreSQL"),
        // Documentation
        ("md", "Markdown"),
        ("rst", "reStructuredText"),
        ("tex", "LaTeX"),
        ("adoc", "AsciiDoc"),
        // Other Languages
        ("r", "R"),
        ("dart", "Dart"),
        ("m", "Objective-C"),
        ("mm", "Objective-C++"),
        ("clj", "Clojure"),
        ("fs", "F#"),
        ("fsx", "F#"),
        ("erl", "Erlang"),
        ("ex", "Elixir"),
        ("exs", "Elixir"),
        ("elm", "Elm"),
        ("hs", "Haskell"),
        ("jl", "Julia"),
    ];

    /// skip Common binary and generated file extensions
    pub const BINARY_EXTENSIONS: &[&'static str] = &[
        "exe", "dll", "so", "dylib", "class", "jar", "war", "ear", "pyc", "pyo", "pyd", "o", "obj",
        "a", "lib", "out",
    ];

    // pub const MIN_FILE_SIZE: u64 = 1; // Skip empty files

    // pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

    /// Files and directories to exclude from analysis
    pub const EXCLUDED_FILES_AND_FOLDERS: &[&'static str] = &[
        // Dependencies
        "node_modules",
        "target",
        "vendor",
        "packages",
        "bower_components",
        // Build outputs
        "dist",
        "build",
        "out",
        "bin",
        "obj",
        // Version Control
        ".git",
        ".svn",
        ".hg",
        ".bzr",
        // IDE and Editor
        ".idea",
        ".vscode",
        ".vs",
        ".settings",
        ".project",
        ".classpath",
        // Testing and Coverage
        "coverage",
        "__pycache__",
        ".pytest_cache",
        ".nyc_output",
        // Logs and Temporary
        "log",
        "logs",
        "tmp",
        "temp",
        ".cache",
        // Documentation
        "docs/_build",
        "site-packages",
        // Platform specific
        ".DS_Store",
        "Thumbs.db",
    ];
}

impl Constants {
    /// Check if a file extension is associated with a binary file
    pub fn is_binary_extension(ext: &str) -> bool {
        Self::BINARY_EXTENSIONS.contains(&ext.to_lowercase().as_str())
    }

    /// Get the programming language based on   file extension
    pub fn get_language(ext: &str) -> Option<&'static str> {
        Self::LANGUAGE_MAP
            .iter()
            .find(|(e, _)| *e == ext.to_lowercase())
            .map(|(_, lang)| *lang)
    }

    /// Get comment markers for a given language
    pub fn get_comment_markers(language: &str) -> Option<&'static [&'static str]> {
        Self::COMMENT_MARKERS
            .iter()
            .find(|(lang, _)| *lang == language)
            .map(|(_, markers)| *markers)
    }
}

