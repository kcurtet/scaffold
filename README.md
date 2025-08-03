# Scaffold 🚀

A powerful CLI tool for quickly scaffolding React, React Native, and Rust projects with modern configurations and best practices.

## Features

- **React Projects**: Modern React setup with Vite, TypeScript support, and testing configuration
- **React Native Projects**: Complete React Native setup with navigation and TypeScript support
- **Rust Projects**: Enhanced Rust projects with common dependencies and optimized configurations
- **Interactive CLI**: Easy-to-use command-line interface with helpful options
- **Customizable**: Flexible options for different project configurations

## Installation

### Build from Source

```bash
git clone <your-repo-url>
cd scaffold
cargo build --release
```

The binary will be available at `target/release/scaffold`.

### Install Globally

To install the CLI globally (optional):

```bash
cargo install --path .
```

## Usage

### List Available Templates

```bash
scaffold list
```

### Create a React Project

```bash
# Basic React project
scaffold react my-app

# React project with TypeScript
scaffold react my-app --typescript

# React project with TypeScript and testing
scaffold react my-app --typescript --testing
```

### Create a React Native Project

```bash
# Basic React Native project
scaffold react-native MyApp

# React Native with TypeScript
scaffold react-native MyApp --typescript

# React Native with TypeScript and navigation
scaffold react-native MyApp --typescript --navigation
```

### Create a Rust Project

```bash
# Binary project (default)
scaffold rust my-project

# Library project
scaffold rust my-lib --project-type library
```

## Project Templates

### React Projects

React projects are created with:
- **Vite** as the build tool
- Modern **ESNext** configuration
- **TypeScript** support (when enabled)
- **Vitest** for testing (when enabled)
- Pre-configured **ESLint** rules
- Basic **CSS** styling
- Proper project structure:
  ```
  my-app/
  ├── src/
  │   ├── components/
  │   ├── hooks/
  │   ├── utils/
  │   ├── types/
  │   ├── App.tsx
  │   ├── main.tsx
  │   ├── App.css
  │   └── index.css
  ├── public/
  ├── .vscode/
  ├── package.json
  └── tsconfig.json (if TypeScript)
  ```

### React Native Projects

React Native projects include:
- Modern React Native setup
- **TypeScript** support (when enabled)
- **React Navigation** (when enabled)
- Proper project structure:
  ```
  MyApp/
  ├── src/
  │   ├── components/
  │   ├── screens/
  │   ├── navigation/
  │   ├── hooks/
  │   ├── utils/
  │   ├── types/
  │   └── App.tsx
  ├── android/
  ├── ios/
  ├── .vscode/
  ├── package.json
  └── tsconfig.json (if TypeScript)
  ```

### Rust Projects

Rust projects are enhanced with:
- Common useful dependencies (commented out for easy enabling)
- **Performance optimizations** for release builds
- **Development** configurations
- **Criterion** for benchmarking (commented)
- Enhanced Cargo.toml structure

## Command Options

### React Command

```bash
scaffold react <NAME> [OPTIONS]

OPTIONS:
  -t, --typescript    Use TypeScript
  -T, --testing      Include testing setup (Vitest)
  -h, --help         Print help
```

### React Native Command

```bash
scaffold react-native <NAME> [OPTIONS]

OPTIONS:
  -t, --typescript    Use TypeScript
  -n, --navigation   Include navigation setup
  -h, --help         Print help
```

### Rust Command

```bash
scaffold rust <NAME> [OPTIONS]

OPTIONS:
  -p, --project-type <TYPE>    Project type [default: binary] [possible values: binary, library]
  -h, --help                   Print help
```

## Examples

### Create a Full-Featured React App

```bash
scaffold react my-dashboard --typescript --testing
cd my-dashboard
npm install
npm run dev
```

### Create a React Native App with Navigation

```bash
scaffold react-native MyMobileApp --typescript --navigation
cd MyMobileApp
npm install
npx react-native run-android  # or run-ios
```

### Create a Rust Library

```bash
scaffold rust my-utils --project-type library
cd my-utils
cargo test
```

## Dependencies

This tool uses the following main dependencies:
- **clap**: Command-line argument parsing
- **serde**: Serialization framework
- **handlebars**: Template engine (for future enhancements)
- **anyhow**: Error handling
- **tokio**: Async runtime

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Roadmap

- [ ] More project templates (Vue, Svelte, etc.)
- [ ] Custom template system
- [ ] Interactive project setup
- [ ] Git initialization option
- [ ] Package manager choice (npm, yarn, pnpm)
- [ ] CI/CD setup templates
- [ ] Docker configurations
- [ ] Additional Rust project types (CLI, web, etc.)

## Support

If you encounter any issues or have feature requests, please open an issue on the repository.

