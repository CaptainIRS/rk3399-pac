#[doc = "Register `FC_GMD_CONF` reader"]
pub type R = crate::R<FcGmdConfSpec>;
#[doc = "Register `FC_GMD_CONF` writer"]
pub type W = crate::W<FcGmdConfSpec>;
#[doc = "Field `GMDPACKETLINESPACING` reader - Number of line spacing between the transmitted\n\nGMD packets"]
pub type GmdpacketlinespacingR = crate::FieldReader;
#[doc = "Field `GMDPACKETLINESPACING` writer - Number of line spacing between the transmitted\n\nGMD packets"]
pub type GmdpacketlinespacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GMDPACKETSINFRAME` reader - Number of GMD packets per frame or video field\n\n(profile P0)"]
pub type GmdpacketsinframeR = crate::FieldReader;
#[doc = "Field `GMDPACKETSINFRAME` writer - Number of GMD packets per frame or video field\n\n(profile P0)"]
pub type GmdpacketsinframeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Number of line spacing between the transmitted\n\nGMD packets"]
    #[inline(always)]
    pub fn gmdpacketlinespacing(&self) -> GmdpacketlinespacingR {
        GmdpacketlinespacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Number of GMD packets per frame or video field\n\n(profile P0)"]
    #[inline(always)]
    pub fn gmdpacketsinframe(&self) -> GmdpacketsinframeR {
        GmdpacketsinframeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of line spacing between the transmitted\n\nGMD packets"]
    #[inline(always)]
    #[must_use]
    pub fn gmdpacketlinespacing(&mut self) -> GmdpacketlinespacingW<FcGmdConfSpec> {
        GmdpacketlinespacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Number of GMD packets per frame or video field\n\n(profile P0)"]
    #[inline(always)]
    #[must_use]
    pub fn gmdpacketsinframe(&mut self) -> GmdpacketsinframeW<FcGmdConfSpec> {
        GmdpacketsinframeW::new(self, 4)
    }
}
#[doc = "Frame Composer GMD Packet Schedule Configuration Register\n\nThis register configures the number of GMD packets to be inserted per frame (starting\n\nalways in the line where the active Vsync appears) and the line spacing between the\n\ntransmitted GMD packets.\n\nNote that for profile P0 (refer to the HDMI 1.4b specification) this register should only\n\nindicate one GMD packet to be inserted per video field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcGmdConfSpec;
impl crate::RegisterSpec for FcGmdConfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_gmd_conf::R`](R) reader structure"]
impl crate::Readable for FcGmdConfSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_gmd_conf::W`](W) writer structure"]
impl crate::Writable for FcGmdConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_GMD_CONF to value 0x10"]
impl crate::Resettable for FcGmdConfSpec {
    const RESET_VALUE: u8 = 0x10;
}
