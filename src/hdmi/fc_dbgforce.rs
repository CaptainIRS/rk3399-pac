#[doc = "Register `FC_DBGFORCE` reader"]
pub type R = crate::R<FcDbgforceSpec>;
#[doc = "Register `FC_DBGFORCE` writer"]
pub type W = crate::W<FcDbgforceSpec>;
#[doc = "Field `FORCEVIDEO` reader - Force fixed video output with FC_DBGTMDSx register\n\ncontents."]
pub type ForcevideoR = crate::BitReader;
#[doc = "Field `FORCEVIDEO` writer - Force fixed video output with FC_DBGTMDSx register\n\ncontents."]
pub type ForcevideoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEAUDIO` reader - Force fixed audio output with FC_DBGAUDxCHx\n\nregister contents."]
pub type ForceaudioR = crate::BitReader;
#[doc = "Field `FORCEAUDIO` writer - Force fixed audio output with FC_DBGAUDxCHx\n\nregister contents."]
pub type ForceaudioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force fixed video output with FC_DBGTMDSx register\n\ncontents."]
    #[inline(always)]
    pub fn forcevideo(&self) -> ForcevideoR {
        ForcevideoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Force fixed audio output with FC_DBGAUDxCHx\n\nregister contents."]
    #[inline(always)]
    pub fn forceaudio(&self) -> ForceaudioR {
        ForceaudioR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force fixed video output with FC_DBGTMDSx register\n\ncontents."]
    #[inline(always)]
    #[must_use]
    pub fn forcevideo(&mut self) -> ForcevideoW<FcDbgforceSpec> {
        ForcevideoW::new(self, 0)
    }
    #[doc = "Bit 4 - Force fixed audio output with FC_DBGAUDxCHx\n\nregister contents."]
    #[inline(always)]
    #[must_use]
    pub fn forceaudio(&mut self) -> ForceaudioW<FcDbgforceSpec> {
        ForceaudioW::new(self, 4)
    }
}
#[doc = "Frame Composer video/audio Force Enable Register\n\nThis register allows to force the controller to output audio and video data the values\n\nconfigured in the FC_DBGAUD and FC_DBGTMDS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgforce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgforce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgforceSpec;
impl crate::RegisterSpec for FcDbgforceSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgforce::R`](R) reader structure"]
impl crate::Readable for FcDbgforceSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgforce::W`](W) writer structure"]
impl crate::Writable for FcDbgforceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGFORCE to value 0"]
impl crate::Resettable for FcDbgforceSpec {
    const RESET_VALUE: u8 = 0;
}
