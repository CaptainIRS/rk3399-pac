#[doc = "Register `FC_VSDSIZE` reader"]
pub type R = crate::R<FcVsdsizeSpec>;
#[doc = "Register `FC_VSDSIZE` writer"]
pub type W = crate::W<FcVsdsizeSpec>;
#[doc = "Field `VSDSIZE` reader - Packet size as described in the HDMI Vendor Specific InfoFrame (from the HDMI specification)."]
pub type VsdsizeR = crate::FieldReader;
#[doc = "Field `VSDSIZE` writer - Packet size as described in the HDMI Vendor Specific InfoFrame (from the HDMI specification)."]
pub type VsdsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Packet size as described in the HDMI Vendor Specific InfoFrame (from the HDMI specification)."]
    #[inline(always)]
    pub fn vsdsize(&self) -> VsdsizeR {
        VsdsizeR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Packet size as described in the HDMI Vendor Specific InfoFrame (from the HDMI specification)."]
    #[inline(always)]
    #[must_use]
    pub fn vsdsize(&mut self) -> VsdsizeW<FcVsdsizeSpec> {
        VsdsizeW::new(self, 0)
    }
}
#[doc = "Packet size as described in the HDMI Vendor Specific InfoFrame (from the HDMI specification).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcVsdsizeSpec;
impl crate::RegisterSpec for FcVsdsizeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_vsdsize::R`](R) reader structure"]
impl crate::Readable for FcVsdsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_vsdsize::W`](W) writer structure"]
impl crate::Writable for FcVsdsizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_VSDSIZE to value 0x1b"]
impl crate::Resettable for FcVsdsizeSpec {
    const RESET_VALUE: u8 = 0x1b;
}
