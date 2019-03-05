use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureId(usize);

impl TextureId {
    pub fn id(self) -> usize {
        self.0
    }
}

impl From<usize> for TextureId {
    fn from(id: usize) -> Self {
        TextureId(id)
    }
}

impl<T> From<*const T> for TextureId {
    fn from(ptr: *const T) -> Self {
        TextureId(ptr as usize)
    }
}

impl<T> From<*mut T> for TextureId {
    fn from(ptr: *mut T) -> Self {
        TextureId(ptr as usize)
    }
}

/// Generic texture mapping for use by renderers.
#[derive(Debug, Default)]
pub struct Textures<T> {
    textures: HashMap<usize, T>,
    next: usize,
}

impl<T> Textures<T> {
    pub fn new() -> Self {
        Textures {
            textures: HashMap::new(),
            next: 0,
        }
    }

    pub fn insert(&mut self, texture: T) -> TextureId {
        let id = self.next;
        self.textures.insert(id, texture);
        self.next += 1;
        TextureId::from(id)
    }

    pub fn replace(&mut self, id: TextureId, texture: T) -> Option<T> {
        self.textures.insert(id.0, texture)
    }

    pub fn remove(&mut self, id: TextureId) -> Option<T> {
        self.textures.remove(&id.0)
    }

    pub fn get(&self, id: TextureId) -> Option<&T> {
        self.textures.get(&id.0)
    }
}