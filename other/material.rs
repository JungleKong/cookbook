use std::ffi::CString;
use std::{ffi, ptr};
use std::cell::RefCell;
use std::rc::Rc;
use crate::{bindgen, rust_filament::backend::CullingMode};

use super::{
    BlendingMode, Engine, Interpolation, MaterialDomain, MaterialInstance, ReflectionMode,
    RefractionMode, RefractionType, Shading, TransparencyMode, VertexDomain,
};

pub struct MaterialBuilder {
    native: ptr::NonNull<bindgen::filament_Material_Builder>,
}

impl MaterialBuilder {
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn native(&self) -> *const bindgen::filament_Material_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn native_mut(&mut self) -> *mut bindgen::filament_Material_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn try_from_native(
        native: *mut bindgen::filament_Material_Builder,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(MaterialBuilder { native: ptr })
    }

    #[inline]
    pub unsafe fn new() -> Option<Self> {
        Self::try_from_native(bindgen::helper_filament_material_builder_create())
    }

    #[inline]
    pub unsafe fn package<'a>(&mut self, payload: &'a [u8]) -> &mut Self {
        bindgen::helper_filament_material_builder_package(
            self.native_mut(),
            payload.as_ptr() as *const _,
            payload.len(),
        );
        self
    }

    #[inline]
    pub unsafe fn build(&mut self, engine: Rc<RefCell<Engine>>) -> Option<Material> {
        Material::try_from_native(bindgen::helper_filament_material_builder_build(
            self.native_mut(),
            engine.borrow_mut().native_mut(),
        ))
    }
}

impl Drop for MaterialBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { bindgen::helper_filament_material_builder_delete(self.native_mut()) }
    }
}

#[derive(Clone)]
pub struct Material {
    native: ptr::NonNull<bindgen::filament_Material>,
}

impl Material {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_Material {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_Material {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_Material) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Material { native: ptr })
    }
}

impl Material {
    #[inline]
    pub unsafe fn create_instance(&self) -> Option<MaterialInstance> {
        MaterialInstance::try_from_native(bindgen::helper_filament_material_create_instance(
            self.native(),
            ptr::null(),
        ))
    }

    #[inline]
    pub unsafe fn create_instance_name(
        &self,
        name: impl AsRef<str>,
    ) -> Option<MaterialInstance> {
        let c_name = 
        if let Ok(c_n) = ffi::CString::new(name.as_ref()) {
            c_n
        }
        else {
            CString::default()
        };

        MaterialInstance::try_from_native(
            bindgen::helper_filament_material_create_instance(self.native(), c_name.as_ptr()),
        )
    }

    #[inline]
    pub unsafe fn get_name(&self) -> String {
        ffi::CStr::from_ptr(bindgen::helper_filament_material_get_name(self.native()))
            .to_string_lossy()
            .to_string()
    }

    #[inline]
    pub unsafe fn get_shading(&self) -> Shading {
        Shading::from(bindgen::helper_filament_material_get_shading(self.native()))
    }
    #[inline]
    pub unsafe fn get_interpolation(&self) -> Interpolation {
        Interpolation::from(bindgen::helper_filament_material_get_interpolation(self.native()))
    }
    #[inline]
    pub unsafe fn get_blending_mode(&self) -> BlendingMode {
        BlendingMode::from(bindgen::helper_filament_material_get_blending_mode(self.native()))
    }
    #[inline]
    pub unsafe fn get_vertex_domain(&self) -> VertexDomain {
        VertexDomain::from(bindgen::helper_filament_material_get_vertex_domain(self.native()))
    }
    #[inline]
    pub unsafe fn get_material_domain(&self) -> MaterialDomain {
        MaterialDomain::from(bindgen::helper_filament_material_get_material_domain(self.native()))
    }
    #[inline]
    pub unsafe fn get_culling_mode(&self) -> CullingMode {
        CullingMode::from(bindgen::helper_filament_material_get_culling_mode(self.native()))
    }
    #[inline]
    pub unsafe fn get_transparency_mode(&self) -> TransparencyMode {
        TransparencyMode::from(bindgen::helper_filament_material_get_transparency_mode(self.native()))
    }
    #[inline]
    pub unsafe fn is_color_write_enabled(&self) -> bool {
        bindgen::helper_filament_material_is_color_write_enabled(self.native())
    }
    #[inline]
    pub unsafe fn is_depth_write_enabled(&self) -> bool {
        bindgen::helper_filament_material_is_depth_write_enabled(self.native())
    }
    #[inline]
    pub unsafe fn is_depth_culling_enabled(&self) -> bool {
        bindgen::helper_filament_material_is_depth_culling_enabled(self.native())
    }
    #[inline]
    pub unsafe fn is_double_sided(&self) -> bool {
        bindgen::helper_filament_material_is_double_sided(self.native())
    }
    #[inline]
    pub unsafe fn is_alpha_to_coverage_enabled(&self) -> bool {
        bindgen::helper_filament_material_is_alpha_to_coverage_enabled(self.native())
    }
    #[inline]
    pub unsafe fn get_mask_threshold(&self) -> f32 {
        bindgen::helper_filament_material_get_mask_threshold(self.native())
    }
    #[inline]
    pub unsafe fn has_shadow_multiplier(&self) -> bool {
        bindgen::helper_filament_material_has_shadow_multiplier(self.native())
    }
    #[inline]
    pub unsafe fn has_specular_anti_aliasing(&self) -> bool {
        bindgen::helper_filament_material_has_specular_anti_alias(self.native())
    }
    #[inline]
    pub unsafe fn get_specular_anti_aliasing_variance(&self) -> f32 {
        bindgen::helper_filament_material_get_specular_anti_alias_variance(self.native())
    }
    #[inline]
    pub unsafe fn get_specular_anti_aliasing_threshold(&self) -> f32 {
        bindgen::helper_filament_material_get_specular_anti_aliasing_threshold(self.native())
    }
    #[inline]
    pub unsafe fn get_parameter_count(&self) -> usize {
        bindgen::helper_filament_material_get_parameter_count(self.native())
    }
    #[inline]
    pub unsafe fn get_parameters(
        &self,
        parameters: *mut bindgen::filament_Material_ParameterInfo,
        count: usize,
    ) -> usize {
        bindgen::helper_filament_material_get_parameters(self.native(), parameters, count)
    }
    #[inline]
    pub unsafe fn get_required_attributes(&self) -> u32 {
        bindgen::helper_filament_material_get_required_attributes(self.native())
    }
    #[inline]
    pub unsafe fn get_refraction_mode(&self) -> RefractionMode {
        RefractionMode::from(bindgen::helper_filament_material_get_refraction_mode(self.native()))
    }
    #[inline]
    pub unsafe fn get_refraction_type(&self) -> RefractionType {
        RefractionType::from(bindgen::helper_filament_material_get_refraction_type(self.native()))
    }
    #[inline]
    pub unsafe fn get_reflection_mode(&self) -> ReflectionMode {
        ReflectionMode::from(bindgen::helper_filament_material_get_reflection_mode(self.native()))
    }

    // todo with Material::create_instance_name. How to deal with name->c_name?
    #[inline]
    pub unsafe fn has_parameter(&self, name: impl AsRef<str>) -> Result<bool, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;

        Ok(bindgen::helper_filament_material_has_parameter(
            self.native(),
            c_name.as_ptr(),
        ))
    }

    // todo with name? Material::has_parameter
    #[inline]
    pub unsafe fn is_sampler(&self, name: *const ::std::os::raw::c_char) -> bool {
        bindgen::helper_filament_material_is_sampler(self.native(), name)
    }

    // #[inline]
    // pub unsafe fn get_default_instance(&mut self) -> Option<MaterialInstance> {
    //     todo!("Material::try_from_native() need pass a mut pointer");
    //     MaterialInstance::try_from_native(bindgen::helper_filament_material_get_default_instance_mut(
    //         self.native_mut(),
    //     ))
    // }

    #[inline]
    pub unsafe fn get_default_instance(&mut self) -> Option<MaterialInstance> {
        MaterialInstance::try_from_native(bindgen::helper_filament_material_get_default_instance_mut(
            self.native_mut(),
        ))
    }

    // #[inline]
    // pub unsafe fn compile(&mut self, backend::CallbackHandler, ..) {

    // }
}


#[cfg(test)]
mod test_rust_filament {
    use super::*;

    #[allow(non_snake_case)] 
    #[test]
    fn test_bindgen_layout_filament_Material_Builder() {
        assert_eq!(
            ::core::mem::size_of::<bindgen::filament_Material_Builder>(),
            8usize,
            concat!("Size of: ", stringify!(bindgen::filament_Material_Builder))
        );

        assert_eq!(
            ::core::mem::align_of::<bindgen::filament_Material_Builder>(),
            8usize,
            concat!("Alignment of ", stringify!(bindgen::filament_Material_Builder))
        );
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_bindgen_layout_filament_Material() {
        assert_eq!(
            ::core::mem::size_of::<bindgen::filament_Material>(),
            1usize,
            concat!("Size of: ", stringify!(bindgen::filament_Material))
        );

        assert_eq!(
            ::core::mem::align_of::<bindgen::filament_Material>(),
            1usize,
            concat!("Alignment of ", stringify!(bindgen::filament_Material))
        );
    }
}
