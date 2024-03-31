#[doc = "Register `GHWPARAMS4` reader"]
pub type R = crate::R<Ghwparams4Spec>;
#[doc = "Field `GHWPARAMS4` reader - Global Hardware Parameters Register 4\n\nGlobal Hardware Parameters Register 4"]
pub type Ghwparams4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Global Hardware Parameters Register 4\n\nGlobal Hardware Parameters Register 4"]
    #[inline(always)]
    pub fn ghwparams4(&self) -> Ghwparams4R {
        Ghwparams4R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwparams4Spec;
impl crate::RegisterSpec for Ghwparams4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwparams4::R`](R) reader structure"]
impl crate::Readable for Ghwparams4Spec {}
#[doc = "`reset()` method sets GHWPARAMS4 to value 0x4782_2008"]
impl crate::Resettable for Ghwparams4Spec {
    const RESET_VALUE: u32 = 0x4782_2008;
}
