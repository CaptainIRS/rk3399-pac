#[doc = "Register `FC_DATAUTO1` reader"]
pub type R = crate::R<FcDatauto1Spec>;
#[doc = "Register `FC_DATAUTO1` writer"]
pub type W = crate::W<FcDatauto1Spec>;
#[doc = "Field `AUTO_FRAME_INTERPOLATION` reader - Packet frame interpolation for automatic packet\n\nscheduling"]
pub type AutoFrameInterpolationR = crate::FieldReader;
#[doc = "Field `AUTO_FRAME_INTERPOLATION` writer - Packet frame interpolation for automatic packet\n\nscheduling"]
pub type AutoFrameInterpolationW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Packet frame interpolation for automatic packet\n\nscheduling"]
    #[inline(always)]
    pub fn auto_frame_interpolation(&self) -> AutoFrameInterpolationR {
        AutoFrameInterpolationR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Packet frame interpolation for automatic packet\n\nscheduling"]
    #[inline(always)]
    #[must_use]
    pub fn auto_frame_interpolation(&mut self) -> AutoFrameInterpolationW<FcDatauto1Spec> {
        AutoFrameInterpolationW::new(self, 0)
    }
}
#[doc = "Frame Composer Data Island Auto Packet Scheduling Register 1\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for SPD, VSD, ISRC2,\n\nISRC1 and ACP packet insertion on data island when FC is on RDRB mode for the listed\n\npackets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_datauto1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datauto1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDatauto1Spec;
impl crate::RegisterSpec for FcDatauto1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_datauto1::R`](R) reader structure"]
impl crate::Readable for FcDatauto1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_datauto1::W`](W) writer structure"]
impl crate::Writable for FcDatauto1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DATAUTO1 to value 0"]
impl crate::Resettable for FcDatauto1Spec {
    const RESET_VALUE: u8 = 0;
}
