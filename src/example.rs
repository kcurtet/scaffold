// Cargo.toml dependencies needed:
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// tokio = { version = "1.0", features = ["full"] }
// anyhow = "1.0"
// handlebars = "4.0"

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use handlebars::Handlebars;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "scaffold")]
#[command(about = "A CLI tool for scaffolding React/React Native projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new React project
    React {
        /// Project name
        name: String,
        /// Use TypeScript
        #[arg(short, long)]
        typescript: bool,
        /// Include testing setup
        #[arg(short = 'T', long)]
        testing: bool,
    },
    /// Create a new React Native project
    ReactNative {
        /// Project name
        name: String,
        /// Use TypeScript
        #[arg(short, long)]
        typescript: bool,
        /// Include navigation setup
        #[arg(short, long)]
        navigation: bool,
    },
    /// Create a new Rust project
    Rust {
        /// Project name
        name: String,
        /// Project type
        #[arg(short, long, default_value = "binary")]
        project_type: String,
    },
    /// List available templates
    List,
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    name: String,
    typescript: bool,
    testing: bool,
    navigation: bool,
}

struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    fn new() -> Self {
        let mut handlebars = Handlebars::new();
        // Register templates here
        Self { handlebars }
    }

    fn render_template(&self, template: &str, data: &ProjectConfig) -> Result<String> {
        self.handlebars.render_template(template, data)
            .context("Failed to render template")
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::React { name, typescript, testing } => {
            create_react_project(&name, typescript, testing)?;
        }
        Commands::ReactNative { name, typescript, navigation } => {
            create_react_native_project(&name, typescript, navigation)?;
        }
        Commands::Rust { name, project_type } => {
            create_rust_project(&name, &project_type)?;
        }
        Commands::List => {
            list_templates();
        }
    }

    Ok(())
}

fn create_react_project(name: &str, typescript: bool, testing: bool) -> Result<()> {
    println!("🚀 Creating React project: {}", name);
    
    let project_path = Path::new(name);
    
    if project_path.exists() {
        anyhow::bail!("Directory {} already exists", name);
    }

    // Create project structure
    create_directory_structure(project_path, &get_react_structure())?;
    
    // Generate package.json
    let package_json = generate_package_json(name, typescript, testing, false)?;
    fs::write(project_path.join("package.json"), package_json)?;
    
    // Generate tsconfig if TypeScript
    if typescript {
        let tsconfig = generate_tsconfig()?;
        fs::write(project_path.join("tsconfig.json"), tsconfig)?;
    }
    
    // Generate main files
    generate_react_files(project_path, typescript)?;
    
    println!("✅ React project '{}' created successfully!", name);
    println!("📁 Next steps:");
    println!("   cd {}", name);
    println!("   npm install");
    println!("   npm start");
    
    Ok(())
}

fn create_react_native_project(name: &str, typescript: bool, navigation: bool) -> Result<()> {
    println!("📱 Creating React Native project: {}", name);
    
    let project_path = Path::new(name);
    
    if project_path.exists() {
        anyhow::bail!("Directory {} already exists", name);
    }

    create_directory_structure(project_path, &get_react_native_structure())?;
    
    let package_json = generate_package_json(name, typescript, false, navigation)?;
    fs::write(project_path.join("package.json"), package_json)?;
    
    if typescript {
        let tsconfig = generate_tsconfig()?;
        fs::write(project_path.join("tsconfig.json"), tsconfig)?;
    }
    
    generate_react_native_files(project_path, typescript, navigation)?;
    
    println!("✅ React Native project '{}' created successfully!", name);
    println!("📁 Next steps:");
    println!("   cd {}", name);
    println!("   npm install");
    println!("   npx react-native run-android  # or run-ios");
    
    Ok(())
}

fn create_rust_project(name: &str, project_type: &str) -> Result<()> {
    println!("🦀 Creating Rust project: {}", name);
    
    let project_path = Path::new(name);
    
    if project_path.exists() {
        anyhow::bail!("Directory {} already exists", name);
    }

    // Use cargo to create the project
    let output = std::process::Command::new("cargo")
        .args(&["new", name, "--name", name])
        .args(if project_type == "library" { vec!["--lib"] } else { vec![] })
        .output()
        .context("Failed to execute cargo new")?;

    if !output.status.success() {
        anyhow::bail!("Cargo new failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Add custom Cargo.toml configurations
    enhance_cargo_toml(project_path)?;
    
    println!("✅ Rust project '{}' created successfully!", name);
    println!("📁 Next steps:");
    println!("   cd {}", name);
    println!("   cargo run  # or cargo build");
    
    Ok(())
}

fn create_directory_structure(base_path: &Path, structure: &[&str]) -> Result<()> {
    fs::create_dir_all(base_path)?;
    
    for dir in structure {
        let dir_path = base_path.join(dir);
        fs::create_dir_all(dir_path)?;
    }
    
    Ok(())
}

fn get_react_structure() -> Vec<&'static str> {
    vec![
        "src",
        "src/components",
        "src/hooks",
        "src/utils",
        "src/types",
        "public",
        ".vscode",
    ]
}

fn get_react_native_structure() -> Vec<&'static str> {
    vec![
        "src",
        "src/components",
        "src/screens",
        "src/navigation",
        "src/hooks",
        "src/utils",
        "src/types",
        "android",
        "ios",
        ".vscode",
    ]
}

fn generate_package_json(name: &str, typescript: bool, testing: bool, navigation: bool) -> Result<String> {
    let mut dependencies = vec![
        ("react", "^18.2.0"),
    ];
    
    let mut dev_dependencies = vec![
        ("@vitejs/plugin-react", "^4.0.3"),
        ("vite", "^4.4.5"),
    ];
    
    if typescript {
        dev_dependencies.extend_from_slice(&[
            ("typescript", "^5.0.2"),
            ("@types/react", "^18.2.15"),
            ("@types/react-dom", "^18.2.7"),
        ]);
    }
    
    if testing {
        dev_dependencies.extend_from_slice(&[
            ("@testing-library/react", "^13.4.0"),
            ("@testing-library/jest-dom", "^5.17.0"),
            ("vitest", "^0.34.4"),
        ]);
    }
    
    if navigation {
        dependencies.extend_from_slice(&[
            ("@react-navigation/native", "^6.1.7"),
            ("@react-navigation/stack", "^6.3.17"),
        ]);
    }
    
    // Build JSON string (simplified - in real implementation use serde_json)
    let deps_str = dependencies.iter()
        .map(|(name, version)| format!(r#"    "{}": "{}""#, name, version))
        .collect::<Vec<_>>()
        .join(",\n");
        
    let dev_deps_str = dev_dependencies.iter()
        .map(|(name, version)| format!(r#"    "{}": "{}""#, name, version))
        .collect::<Vec<_>>()
        .join(",\n");
    
    Ok(format!(r#"{{
  "name": "{}",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {{
    "dev": "vite",
    "build": "vite build",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "preview": "vite preview"{}
  }},
  "dependencies": {{
{}
  }},
  "devDependencies": {{
{}
  }}
}}"#, 
        name,
        if testing { r#",
    "test": "vitest""# } else { "" },
        deps_str,
        dev_deps_str
    ))
}

fn generate_tsconfig() -> Result<String> {
    Ok(r#"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}"#.to_string())
}

fn generate_react_files(project_path: &Path, typescript: bool) -> Result<()> {
    let ext = if typescript { "tsx" } else { "jsx" };
    
    let app_content = format!(r#"import React from 'react';
import './App.css';

function App() {{
  return (
    <div className="App">
      <header className="App-header">
        <h1>Welcome to your new React project!</h1>
        <p>Edit src/App.{} and save to reload.</p>
      </header>
    </div>
  );
}}

export default App;"#, ext);
    
    fs::write(project_path.join(format!("src/App.{}", ext)), app_content)?;
    
    let main_content = format!(r#"import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.{}';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);"#, ext);
    
    fs::write(project_path.join(format!("src/main.{}", if typescript { "tsx" } else { "jsx" })), main_content)?;
    
    // Basic CSS
    let css_content = r#"body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.App {
  text-align: center;
}

.App-header {
  background-color: #282c34;
  padding: 20px;
  color: white;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  font-size: calc(10px + 2vmin);
}"#;
    
    fs::write(project_path.join("src/App.css"), css_content)?;
    fs::write(project_path.join("src/index.css"), "/* Global styles */")?;
    
    Ok(())
}

fn generate_react_native_files(project_path: &Path, typescript: bool, _navigation: bool) -> Result<()> {
    let ext = if typescript { "tsx" } else { "jsx" };
    
    let app_content = format!(r#"import React from 'react';
import {{
  SafeAreaView,
  ScrollView,
  StatusBar,
  StyleSheet,
  Text,
  View,
}} from 'react-native';

function App(){} {{
  return (
    <SafeAreaView style={{styles.container}}>
      <StatusBar barStyle="dark-content" />
      <ScrollView contentInsetAdjustmentBehavior="automatic">
        <View style={{styles.body}}>
          <Text style={{styles.title}}>Welcome to React Native!</Text>
          <Text style={{styles.subtitle}}>Your project is ready to go.</Text>
        </View>
      </ScrollView>
    </SafeAreaView>
  );
}}

const styles = StyleSheet.create({{
  container: {{
    flex: 1,
  }},
  body: {{
    backgroundColor: '#fff',
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  }},
  title: {{
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 10,
  }},
  subtitle: {{
    fontSize: 16,
    color: '#666',
  }},
}});

export default App;"#, if typescript { ": React.FC" } else { "" });
    
    fs::write(project_path.join(format!("src/App.{}", ext)), app_content)?;
    
    Ok(())
}

fn enhance_cargo_toml(project_path: &Path) -> Result<()> {
    let cargo_path = project_path.join("Cargo.toml");
    let mut content = fs::read_to_string(&cargo_path)?;
    
    // Add some common useful dependencies and configurations
    content.push_str(r#"

# Common useful dependencies (uncomment as needed)
[dependencies]
# clap = { version = "4.0", features = ["derive"] }
# serde = { version = "1.0", features = ["derive"] }
# serde_json = "1.0"
# tokio = { version = "1.0", features = ["full"] }
# anyhow = "1.0"

[dev-dependencies]
# criterion = "0.5"

# Performance optimizations
[profile.release]
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
debug = true
"#);
    
    fs::write(cargo_path, content)?;
    Ok(())
}

fn list_templates() {
    println!("📋 Available templates:");
    println!("  react         - React web application");
    println!("  react-native  - React Native mobile application");
    println!("  rust          - Rust application or library");
    println!();
    println!("Use --help with any command for more options.");
}
