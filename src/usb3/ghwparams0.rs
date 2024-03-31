#[doc = "Register `GHWPARAMS0` reader"]
pub type R = crate::R<Ghwparams0Spec>;
#[doc = "Field `GHWPARAMS0` reader - Global Hardware Parameters Register 0\n\nGlobal Hardware Parameters Register 0"]
pub type Ghwparams0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Global Hardware Parameters Register 0\n\nGlobal Hardware Parameters Register 0"]
    #[inline(always)]
    pub fn ghwparams0(&self) -> Ghwparams0R {
        Ghwparams0R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwparams0Spec;
impl crate::RegisterSpec for Ghwparams0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwparams0::R`](R) reader structure"]
impl crate::Readable for Ghwparams0Spec {}
#[doc = "`reset()` method sets GHWPARAMS0 to value 0x2020_400a"]
impl crate::Resettable for Ghwparams0Spec {
    const RESET_VALUE: u32 = 0x2020_400a;
}
