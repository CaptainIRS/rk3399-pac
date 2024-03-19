#[doc = "Register `USB3_GHWPARAMS3` reader"]
pub type R = crate::R<Usb3Ghwparams3Spec>;
#[doc = "Field `GHWPARAMS3` reader - Global Hardware Parameters Register 3\n\nGlobal Hardware Parameters Register 3"]
pub type Ghwparams3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Global Hardware Parameters Register 3\n\nGlobal Hardware Parameters Register 3"]
    #[inline(always)]
    pub fn ghwparams3(&self) -> Ghwparams3R {
        Ghwparams3R::new(self.bits)
    }
}
#[doc = "Global Hardware Parameters Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Ghwparams3Spec;
impl crate::RegisterSpec for Usb3Ghwparams3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_ghwparams3::R`](R) reader structure"]
impl crate::Readable for Usb3Ghwparams3Spec {}
#[doc = "`reset()` method sets USB3_GHWPARAMS3 to value 0x069c_d085"]
impl crate::Resettable for Usb3Ghwparams3Spec {
    const RESET_VALUE: u32 = 0x069c_d085;
}
