pub struct Window {
    window: minifb::Window,
    framebuffer: FrameBuffer,
}

pub struct FrameBuffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(name, width, height, options).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Window {
            window,
            framebuffer: FrameBuffer::new(width, height),
        }
    }

    pub fn framebuffer(&mut self) -> &mut FrameBuffer {
        &mut self.framebuffer
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn limit_60_fps(&mut self) {
        self.window
            .limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(
                &self.framebuffer.data,
                self.framebuffer.width(),
                self.framebuffer.height(),
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        let (width, height) = self.window.get_size();
        if width != self.framebuffer.width() || height != self.framebuffer.height() {
            self.framebuffer = FrameBuffer::new(width, height);
        }
    }
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, value: u32) {
        let (width, height) = (self.width as i32, self.height as i32);

        let screen_x = width / 2 + x as i32;
        let screen_y = height / 2 - y as i32 - 1;

        if (screen_x < 0) | (screen_x >= width) | (screen_y < 0) | (screen_y >= height) {
            return;
        }
        self.data[screen_x as usize + screen_y as usize * self.width] = value;
    }

    pub fn clear(&mut self, value: u32) {
        for i in 0..self.data.len() {
            self.data[i] = value;
        }
    }
}
