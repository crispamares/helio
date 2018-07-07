# Helio

**WARNING** : Don't use it... just for experimenting at this time

Helio is a visualization library for Rust. 

- Should be fast:
    - Columnar layout for the data
    - Layers for skip rendering

## Roadmap

- Rendering for Marks
    - rect
    - circle
    - line
    - path
    - arc
- Scales
- Layers
- Axis
- Legends
- Interactions

- [x] First backend in SVG for easy debugging
- [ ] Create a raster backend in Rust with the API of html5 canvas
- [ ] Use the html5 canvas so it can be compiled to Wasm and draw using the browser's canvas.