#[doc = "Register `GHWPARAMS8` reader"]
pub type R = crate::R<Ghwparams8Spec>;
#[doc = "Field `GHWPARAMS8_32_0` reader - ghwparams8_32_0\n\nghwparams8_32_0"]
pub type Ghwparams8_32_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ghwparams8_32_0\n\nghwparams8_32_0"]
    #[inline(always)]
    pub fn ghwparams8_32_0(&self) -> Ghwparams8_32_0R {
        Ghwparams8_32_0R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwparams8Spec;
impl crate::RegisterSpec for Ghwparams8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwparams8::R`](R) reader structure"]
impl crate::Readable for Ghwparams8Spec {}
#[doc = "`reset()` method sets GHWPARAMS8 to value 0x077c"]
impl crate::Resettable for Ghwparams8Spec {
    const RESET_VALUE: u32 = 0x077c;
}
