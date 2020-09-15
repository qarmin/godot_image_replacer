## Godot image replacer
This repository contains app which replace all `jpg` and `png` images with another one with 1x1 size and random color.  
This for big projects like [Tps Demo](https://github.com/godotengine/tps-demo) allows to import project a lot of faster.  
It is helpful e.g. in testing project with help of Valgrind or sanitizers because importing images take a lot of time and every second counts.

### Usage
```
godot_image_replacer /home/rafal/Project/tps-demo
```

### License
MIT