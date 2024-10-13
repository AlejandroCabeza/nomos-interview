use viuer::Config;

pub fn get_image_config() -> Config {
    Config {
        width: Some(48),
        height: Some(48),
        ..Default::default()
    }
}

pub fn debug_image_config(image_config: &Config) -> String {
    format!("{}-{}", image_config.x, image_config.y)
}

pub fn clone_image_config(image_config: &Config) -> Config {
    Config {
        transparent: image_config.transparent,
        absolute_offset: image_config.absolute_offset,
        x: image_config.x,
        y: image_config.y,
        restore_cursor: image_config.restore_cursor,
        width: image_config.width,
        height: image_config.height,
        truecolor: image_config.truecolor,
        use_kitty: image_config.use_kitty,
        use_iterm: image_config.use_iterm,
        #[cfg(feature = "viuer_sixel")]
        use_sixel: image_config.use_sixel,
    }
}
