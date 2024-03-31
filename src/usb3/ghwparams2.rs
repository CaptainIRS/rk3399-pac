#[doc = "Register `GHWPARAMS2` reader"]
pub type R = crate::R<Ghwparams2Spec>;
#[doc = "Field `GHWPARAMS2` reader - Global Hardware Parameters Register 2\n\nGlobal Hardware Parameters Register 2"]
pub type Ghwparams2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Global Hardware Parameters Register 2\n\nGlobal Hardware Parameters Register 2"]
    #[inline(always)]
    pub fn ghwparams2(&self) -> Ghwparams2R {
        Ghwparams2R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwparams2Spec;
impl crate::RegisterSpec for Ghwparams2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwparams2::R`](R) reader structure"]
impl crate::Readable for Ghwparams2Spec {}
#[doc = "`reset()` method sets GHWPARAMS2 to value 0x1234_5678"]
impl crate::Resettable for Ghwparams2Spec {
    const RESET_VALUE: u32 = 0x1234_5678;
}
