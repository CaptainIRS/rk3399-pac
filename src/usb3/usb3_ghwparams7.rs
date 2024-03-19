#[doc = "Register `USB3_GHWPARAMS7` reader"]
pub type R = crate::R<Usb3Ghwparams7Spec>;
#[doc = "Field `GHWPARAMS7` reader - Global Hardware Parameters Register 7\n\nGlobal Hardware Parameters Register 7"]
pub type Ghwparams7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Global Hardware Parameters Register 7\n\nGlobal Hardware Parameters Register 7"]
    #[inline(always)]
    pub fn ghwparams7(&self) -> Ghwparams7R {
        Ghwparams7R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Ghwparams7Spec;
impl crate::RegisterSpec for Usb3Ghwparams7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_ghwparams7::R`](R) reader structure"]
impl crate::Readable for Usb3Ghwparams7Spec {}
#[doc = "`reset()` method sets USB3_GHWPARAMS7 to value 0x0308_0756"]
impl crate::Resettable for Usb3Ghwparams7Spec {
    const RESET_VALUE: u32 = 0x0308_0756;
}
