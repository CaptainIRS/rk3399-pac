#[doc = "Register `GHWPARAMS1` reader"]
pub type R = crate::R<Ghwparams1Spec>;
#[doc = "Field `GHWPARAMS1` reader - Global Hardware Parameters Register 1\n\nGlobal Hardware Parameters Register 1"]
pub type Ghwparams1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Global Hardware Parameters Register 1\n\nGlobal Hardware Parameters Register 1"]
    #[inline(always)]
    pub fn ghwparams1(&self) -> Ghwparams1R {
        Ghwparams1R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwparams1Spec;
impl crate::RegisterSpec for Ghwparams1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwparams1::R`](R) reader structure"]
impl crate::Readable for Ghwparams1Spec {}
#[doc = "`reset()` method sets GHWPARAMS1 to value 0x0160_c93b"]
impl crate::Resettable for Ghwparams1Spec {
    const RESET_VALUE: u32 = 0x0160_c93b;
}
