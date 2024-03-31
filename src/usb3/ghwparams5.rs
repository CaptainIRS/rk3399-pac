#[doc = "Register `GHWPARAMS5` reader"]
pub type R = crate::R<Ghwparams5Spec>;
#[doc = "Field `GHWPARAMS5` reader - Global Hardware Parameters Register 5\n\nGlobal Hardware Parameters Register 5"]
pub type Ghwparams5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Global Hardware Parameters Register 5\n\nGlobal Hardware Parameters Register 5"]
    #[inline(always)]
    pub fn ghwparams5(&self) -> Ghwparams5R {
        Ghwparams5R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwparams5Spec;
impl crate::RegisterSpec for Ghwparams5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwparams5::R`](R) reader structure"]
impl crate::Readable for Ghwparams5Spec {}
#[doc = "`reset()` method sets GHWPARAMS5 to value 0x0420_2088"]
impl crate::Resettable for Ghwparams5Spec {
    const RESET_VALUE: u32 = 0x0420_2088;
}
