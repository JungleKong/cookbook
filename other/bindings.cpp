// =================================================================================================
// filament::Material::Builder
extern "C" filament::Material::Builder *helper_filament_material_builder_create() {
    return new filament::Material::Builder();
}

extern "C" void helper_filament_material_builder_delete(filament::Material::Builder *self) {
    delete self;
}

extern "C" void helper_filament_material_builder_package(filament::Material::Builder *self, const uint8_t *data, size_t size) {
    self->package(data, size);
}

extern "C" filament::Material *helper_filament_material_builder_build(filament::Material::Builder *self, filament::Engine *engine) {
    return self->build(*engine);
}
// =================================================================================================


// =================================================================================================
// filament::Material
extern "C" filament::MaterialInstance *helper_filament_material_create_instance(const filament::Material *self, const char *name) {
    return self->createInstance(name);
}

extern "C" const char *helper_filament_material_get_name(const filament::Material *self) {
    return self->getName();
}

extern "C" filament::Shading helper_filament_material_get_shading(const filament::Material *self) {
    return self->getShading();
}

extern "C" filament::Interpolation helper_filament_material_get_interpolation(const filament::Material *self) {
    return self->getInterpolation();
}

extern "C" filament::BlendingMode helper_filament_material_get_blending_mode(const filament::Material *self) {
    return self->getBlendingMode();
}

extern "C" filament::VertexDomain helper_filament_material_get_vertex_domain(const filament::Material *self) {
    return self->getVertexDomain();
}

extern "C" filament::MaterialDomain helper_filament_material_get_material_domain(const filament::Material *self) {
    return self->getMaterialDomain();
}

extern "C" filament::backend::CullingMode helper_filament_material_get_culling_mode(const filament::Material *self) {
    return self->getCullingMode();
}

extern "C" filament::TransparencyMode helper_filament_material_get_transparency_mode(const filament::Material *self) {
    return self->getTransparencyMode();
}

extern "C" bool helper_filament_material_is_color_write_enabled(const filament::Material *self) {
    return self->isColorWriteEnabled();
}

extern "C" bool helper_filament_material_is_depth_write_enabled(const filament::Material *self) {
    return self->isDepthWriteEnabled();
}

extern "C" bool helper_filament_material_is_depth_culling_enabled(const filament::Material *self) {
    return self->isDepthCullingEnabled();
}

extern "C" bool helper_filament_material_is_double_sided(const filament::Material *self) {
    return self->isDoubleSided();
}

extern "C" bool helper_filament_material_is_alpha_to_coverage_enabled(const filament::Material *self) {
    return self->isAlphaToCoverageEnabled();
}

extern "C" float helper_filament_material_get_mask_threshold(const filament::Material *self) {
    return self->getMaskThreshold();
}

extern "C" bool helper_filament_material_has_shadow_multiplier(const filament::Material *self) {
    return self->hasShadowMultiplier();
}

extern "C" bool helper_filament_material_has_specular_anti_alias(const filament::Material *self) {
    return self->hasSpecularAntiAliasing();
}

extern "C" float helper_filament_material_get_specular_anti_alias_variance(const filament::Material *self) {
    return self->getSpecularAntiAliasingVariance();
}

extern "C" float helper_filament_material_get_specular_anti_aliasing_threshold(const filament::Material *self) {
    return self->getSpecularAntiAliasingThreshold();
}

extern "C" size_t helper_filament_material_get_parameter_count(const filament::Material *self) {
    return self->getParameterCount();
}

extern "C" size_t helper_filament_material_get_parameters(const filament::Material *self, filament::Material::ParameterInfo *parameters, size_t count) {
    self->getParameters(parameters, count);
}

extern "C" filament::AttributeBitset helper_filament_material_get_required_attributes(const filament::Material *self) {
    return self->getRequiredAttributes();
}

extern "C" filament::RefractionMode helper_filament_material_get_refraction_mode(const filament::Material *self) {
    return self->getRefractionMode();
}

extern "C" filament::RefractionType helper_filament_material_get_refraction_type(const filament::Material *self) {
    return self->getRefractionType();
}

extern "C" filament::ReflectionMode helper_filament_material_get_reflection_mode(const filament::Material *self) {
    return self->getReflectionMode();
}

extern "C" bool helper_filament_material_has_parameter(const filament::Material *self, const char *name) {
    return self->hasParameter(name);
}

extern "C" bool helper_filament_material_is_sampler(const filament::Material *self, const char *name) {
    return self->isSampler(name);
}

extern "C" filament::MaterialInstance const *helper_filament_material_get_default_instance(const filament::Material *self) {
    return self->getDefaultInstance();
}

extern "C" filament::MaterialInstance *helper_filament_material_get_default_instance_mut(filament::Material *self) {
    return self->getDefaultInstance();
}
// =================================================================================================
