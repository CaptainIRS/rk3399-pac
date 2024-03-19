#[doc = "Register `DDR_DENALI_CTL_189` reader"]
pub type R = crate::R<DdrDenaliCtl189Spec>;
#[doc = "Register `DDR_DENALI_CTL_189` writer"]
pub type W = crate::W<DdrDenaliCtl189Spec>;
#[doc = "Field `NO_ZQ_INIT` reader - Disable ZQ operations during initialization. Set to 1 to disable."]
pub type NoZqInitR = crate::BitReader;
#[doc = "Field `NO_ZQ_INIT` writer - Disable ZQ operations during initialization. Set to 1 to disable."]
pub type NoZqInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZQCS_ROTATE` reader - For non-LPDDR4 memories, selects whether a ZQCS command will calibrate just one chip select or all chip selects. When rotation is off, all chip selects will be calibrated, requiring a longer time frame, but ZQ calibration will need to be performed less frequently. Set to 1 for rotating CS. For LPDDR4 memories, this parameter is ignored."]
pub type ZqcsRotateR = crate::BitReader;
#[doc = "Field `ZQCS_ROTATE` writer - For non-LPDDR4 memories, selects whether a ZQCS command will calibrate just one chip select or all chip selects. When rotation is off, all chip selects will be calibrated, requiring a longer time frame, but ZQ calibration will need to be performed less frequently. Set to 1 for rotating CS. For LPDDR4 memories, this parameter is ignored."]
pub type ZqcsRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZQ_CAL_START_MAP_0` reader - Defines which chip select(s) will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
pub type ZqCalStartMap0R = crate::FieldReader;
#[doc = "Field `ZQ_CAL_START_MAP_0` writer - Defines which chip select(s) will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
pub type ZqCalStartMap0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ZQ_CAL_LATCH_MAP_0` reader - Defines which chip select(s) will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
pub type ZqCalLatchMap0R = crate::FieldReader;
#[doc = "Field `ZQ_CAL_LATCH_MAP_0` writer - Defines which chip select(s) will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
pub type ZqCalLatchMap0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Disable ZQ operations during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn no_zq_init(&self) -> NoZqInitR {
        NoZqInitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - For non-LPDDR4 memories, selects whether a ZQCS command will calibrate just one chip select or all chip selects. When rotation is off, all chip selects will be calibrated, requiring a longer time frame, but ZQ calibration will need to be performed less frequently. Set to 1 for rotating CS. For LPDDR4 memories, this parameter is ignored."]
    #[inline(always)]
    pub fn zqcs_rotate(&self) -> ZqcsRotateR {
        ZqcsRotateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Defines which chip select(s) will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
    #[inline(always)]
    pub fn zq_cal_start_map_0(&self) -> ZqCalStartMap0R {
        ZqCalStartMap0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Defines which chip select(s) will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
    #[inline(always)]
    pub fn zq_cal_latch_map_0(&self) -> ZqCalLatchMap0R {
        ZqCalLatchMap0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable ZQ operations during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn no_zq_init(&mut self) -> NoZqInitW<DdrDenaliCtl189Spec> {
        NoZqInitW::new(self, 0)
    }
    #[doc = "Bit 8 - For non-LPDDR4 memories, selects whether a ZQCS command will calibrate just one chip select or all chip selects. When rotation is off, all chip selects will be calibrated, requiring a longer time frame, but ZQ calibration will need to be performed less frequently. Set to 1 for rotating CS. For LPDDR4 memories, this parameter is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn zqcs_rotate(&mut self) -> ZqcsRotateW<DdrDenaliCtl189Spec> {
        ZqcsRotateW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Defines which chip select(s) will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cal_start_map_0(&mut self) -> ZqCalStartMap0W<DdrDenaliCtl189Spec> {
        ZqCalStartMap0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Defines which chip select(s) will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cal_latch_map_0(&mut self) -> ZqCalLatchMap0W<DdrDenaliCtl189Spec> {
        ZqCalLatchMap0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_189::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_189::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl189Spec;
impl crate::RegisterSpec for DdrDenaliCtl189Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_189::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl189Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_189::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl189Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_189 to value 0x0101_0000"]
impl crate::Resettable for DdrDenaliCtl189Spec {
    const RESET_VALUE: u32 = 0x0101_0000;
}
