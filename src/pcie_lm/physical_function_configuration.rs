#[doc = "Register `PHYSICAL_FUNCTION_CONFIGURATION` reader"]
pub type R = crate::R<PhysicalFunctionConfigurationSpec>;
#[doc = "Field `F0E` reader - Function 0 Enable \\[F0E\\]
Enable for Function 0. This bit is hardwired to 1."]
pub type F0eR = crate::BitReader;
#[doc = "Field `R` reader - Reserved \\[R\\]
Reserved"]
pub type RR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Function 0 Enable \\[F0E\\]
Enable for Function 0. This bit is hardwired to 1."]
    #[inline(always)]
    pub fn f0e(&self) -> F0eR {
        F0eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved \\[R\\]
Reserved"]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[doc = "Physical Function Configuration Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_function_configuration::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhysicalFunctionConfigurationSpec;
impl crate::RegisterSpec for PhysicalFunctionConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`physical_function_configuration::R`](R) reader structure"]
impl crate::Readable for PhysicalFunctionConfigurationSpec {}
#[doc = "`reset()` method sets PHYSICAL_FUNCTION_CONFIGURATION to value 0x01"]
impl crate::Resettable for PhysicalFunctionConfigurationSpec {
    const RESET_VALUE: u32 = 0x01;
}
