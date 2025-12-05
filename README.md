# GifMaker

A Rust command-line tool that creates animated GIFs from text with custom backgrounds and transitions.

## Description

GifMaker creates animated GIFs with two text phrases that fade in with smooth transitions. The tool generates a colored background video and overlays your text with configurable fonts, colors, and transitions.

## Dependencies

### System Dependencies

**FFmpeg** (version 6.1.1 or newer with the following features):
- `libfreetype` - for text rendering
- `libfontconfig` - for font configuration
- `libass` - for advanced subtitle support
- Color video generation (`color` filter)
- Text overlay (`drawtext` filter)
- Transition effects (`xfade` filter)

### Required FFmpeg Configuration

Ensure your FFmpeg installation includes:
```
--enable-libfreetype
--enable-libfontconfig
--enable-libass
--enable-filter=drawtext
--enable-filter=xfade
--enable-filter=color
```

To check your FFmpeg configuration:
```bash
ffmpeg -version | grep configuration
```

### Rust Dependencies

- `dotenv = "0.15.0"` - for environment variable configuration

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd gifmaker
```

2. Install FFmpeg (Ubuntu/Debian):
```bash
sudo apt update
sudo apt install ffmpeg
```

3. Build the project:
```bash
cargo build --release
```

## Configuration

Create a `.env` file in the project root with the following variables:

```env
# Background colors (ffmpeg -colors for available options)
F_COLOR=DarkGreen
S_COLOR=DarkBlue

# Transition effects (ffmpeg --help filter=xfade for options)
TRANSITION=vertclose

# Animation duration in seconds
DURATION=5

# Font settings
FONT_SIZE=20

# Maximum characters per line (text wrapping)
LINE_LENGTH=20
```

### Available Options

**Colors**: Use any FFmpeg color name or hex value. Run `ffmpeg -colors` for a complete list.

**Transitions**: Available transition effects include:
- `fade` - Fade transition
- `wipe` - Wipe transition
- `vertclose` - Vertical close transition
- `horizclose` - Horizontal close transition
- And many more (run `ffmpeg --help filter=xfade`)

## Usage

GifMaker supports two modes of operation:

### Interactive Mode

1. **Run the application**:
```bash
./target/release/gifmaker
```

2. **Enter your first text phrase** when prompted.

3. **Enter your second text phrase** when prompted.

### Inline Mode

**For automation and scripting:**
```bash
./target/release/gifmaker --first "Your first phrase" --second "Your second phrase"
```

**Example with multi-word phrases:**
```bash
./target/release/gifmaker --first "Hello World! This is awesome" --second "And this is the second phrase"
```

### What Happens Next

The application will:
- Generate a colored background video with transition effects
- Add your text overlay with the configured colors and fonts
- Create the final GIF with a timestamp in the `gif/` directory
- Automatically open the result with your system's default viewer

### Command Line Options

- `--help` - Display detailed help message
- `--restore-config` - Restore default configuration file
- `--first "<text>"` - Specify first phrase (use with --second)
- `--second "<text>"` - Specify second phrase (use with --first)

## Output

The generated GIF files are saved with timestamps to avoid overwriting:

```
gif/gif_2024-12-05_14:30:25.gif
gif/gif_2024-12-05_14:31:10.gif
```

## Examples

### Interactive Mode Example
```bash
$ ./gifmaker
Enter first phrase:
Hello World! This is my first phrase
Enter second phrase:
And this is the second phrase with more text

Your file saved at: /home/user/gifmaker/gif/gif_2024-12-05_14:30:25.gif
```

### Inline Mode Example
```bash
$ ./gifmaker --first "Quick demo" --second "Second phrase here"

Your file saved at: /home/user/gifmaker/gif/gif_2024-12-05_14:31:10.gif
```

### Automation Example
```bash
# Generate multiple GIFs in a script
./gifmaker --first "Starting process" --second "Initialization complete"
./gifmaker --first "Processing data" --second "Almost finished"
./gifmaker --first "Done!" --second "Task completed successfully"
```

![Example of script usage](assets/output.gif)

## Project Structure

```
gifmaker/
├── src/
│   ├── main.rs          # Main application logic
│   ├── gifconfig.rs     # Configuration handling
│   ├── utils.rs         # Utility functions
│   ├── service.rs       # Service functions
│   └── errors.rs        # Error handling
├── .env                 # Environment variables
├── Cargo.toml           # Rust dependencies
├── gif/                 # Output directory
└── README.md            # This file
```

## Error Handling

The application includes comprehensive error handling for:
- Missing FFmpeg installation
- Invalid configuration values
- File I/O operations
- Text processing errors

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request
