#[doc = "Register `V_SYNC_WIDTH_CFG` reader"]
pub type R = crate::R<VSyncWidthCfgSpec>;
#[doc = "Register `V_SYNC_WIDTH_CFG` writer"]
pub type W = crate::W<VSyncWidthCfgSpec>;
#[doc = "Field `V_SYNC_WIDTH_CFG` reader - This is used to specify the number of lines in \n\nVSYNC period. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type VSyncWidthCfgR = crate::FieldReader;
#[doc = "Field `V_SYNC_WIDTH_CFG` writer - This is used to specify the number of lines in \n\nVSYNC period. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type VSyncWidthCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This is used to specify the number of lines in \n\nVSYNC period. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    pub fn v_sync_width_cfg(&self) -> VSyncWidthCfgR {
        VSyncWidthCfgR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This is used to specify the number of lines in \n\nVSYNC period. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn v_sync_width_cfg(&mut self) -> VSyncWidthCfgW<VSyncWidthCfgSpec> {
        VSyncWidthCfgW::new(self, 0)
    }
}
#[doc = "Vertical Sync Width Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_sync_width_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_sync_width_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VSyncWidthCfgSpec;
impl crate::RegisterSpec for VSyncWidthCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`v_sync_width_cfg::R`](R) reader structure"]
impl crate::Readable for VSyncWidthCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`v_sync_width_cfg::W`](W) writer structure"]
impl crate::Writable for VSyncWidthCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets V_SYNC_WIDTH_CFG to value 0"]
impl crate::Resettable for VSyncWidthCfgSpec {
    const RESET_VALUE: u32 = 0;
}
