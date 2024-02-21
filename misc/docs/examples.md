# Examples

## Line

<p align="left">
  <img src="../screenshots/draw_line.png" width="49%" alt="line" />
</p>

```shell
cargo r --example line
```

## Triangle

<p align="left">
  <img src="../screenshots/draw_triangle.png" width="49%" alt="triangle" />
</p>

```shell
cargo r --example triangle
```

## Obj Model Flat Shading

<p align="left">
  <img src="../screenshots/obj_model_wireframe.png" width="49%" alt="wireframe" />
  <img src="../screenshots/obj_model_flag_shading_0.png" width="49%"  alt="random color"/>
  <img src="../screenshots/obj_model_flag_shading_z_buffer.png" width="49%" alt="z-buffer"/>
  <img src="../screenshots/obj_model_diffuse_with_light.png" width="49%" alt="diffuse"/>
  <img src="../screenshots/obj_model_diffuse_perspective.png" width="49%" alt="diffuse"/>
</p>

Press A => flat shading  
Press S => random color  
Press D => wireframe  
Press W => diffuse  
No input => diffuse perspective

```shell
cargo r --example obj_flat_shading
```