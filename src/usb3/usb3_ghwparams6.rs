#[doc = "Register `USB3_GHWPARAMS6` reader"]
pub type R = crate::R<Usb3Ghwparams6Spec>;
#[doc = "Field `GHWPARAMS6` reader - Global Hardware Parameters Register 6\n\nGlobal Hardware Parameters Register 6"]
pub type Ghwparams6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Global Hardware Parameters Register 6\n\nGlobal Hardware Parameters Register 6"]
    #[inline(always)]
    pub fn ghwparams6(&self) -> Ghwparams6R {
        Ghwparams6R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Ghwparams6Spec;
impl crate::RegisterSpec for Usb3Ghwparams6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_ghwparams6::R`](R) reader structure"]
impl crate::Readable for Usb3Ghwparams6Spec {}
#[doc = "`reset()` method sets USB3_GHWPARAMS6 to value 0x077c_8020"]
impl crate::Resettable for Usb3Ghwparams6Spec {
    const RESET_VALUE: u32 = 0x077c_8020;
}
