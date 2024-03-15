#[doc = "Register `DENALI_CTL_190` reader"]
pub type R = crate::R<DenaliCtl190Spec>;
#[doc = "Register `DENALI_CTL_190` writer"]
pub type W = crate::W<DenaliCtl190Spec>;
#[doc = "Field `ZQ_CAL_START_MAP_1` reader - Defines which chip select(s) will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
pub type ZqCalStartMap1R = crate::FieldReader;
#[doc = "Field `ZQ_CAL_START_MAP_1` writer - Defines which chip select(s) will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
pub type ZqCalStartMap1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ZQ_CAL_LATCH_MAP_1` reader - Defines which chip select(s) will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
pub type ZqCalLatchMap1R = crate::FieldReader;
#[doc = "Field `ZQ_CAL_LATCH_MAP_1` writer - Defines which chip select(s) will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
pub type ZqCalLatchMap1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANK_DIFF` reader - Encoded number of banks on the DRAM(s)."]
pub type BankDiffR = crate::FieldReader;
#[doc = "Field `BANK_DIFF` writer - Encoded number of banks on the DRAM(s)."]
pub type BankDiffW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ROW_DIFF` reader - Difference between number of address pins available and number being used."]
pub type RowDiffR = crate::FieldReader;
#[doc = "Field `ROW_DIFF` writer - Difference between number of address pins available and number being used."]
pub type RowDiffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Defines which chip select(s) will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
    #[inline(always)]
    pub fn zq_cal_start_map_1(&self) -> ZqCalStartMap1R {
        ZqCalStartMap1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Defines which chip select(s) will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
    #[inline(always)]
    pub fn zq_cal_latch_map_1(&self) -> ZqCalLatchMap1R {
        ZqCalLatchMap1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Encoded number of banks on the DRAM(s)."]
    #[inline(always)]
    pub fn bank_diff(&self) -> BankDiffR {
        BankDiffR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Difference between number of address pins available and number being used."]
    #[inline(always)]
    pub fn row_diff(&self) -> RowDiffR {
        RowDiffR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines which chip select(s) will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cal_start_map_1(&mut self) -> ZqCalStartMap1W<DenaliCtl190Spec> {
        ZqCalStartMap1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Defines which chip select(s) will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cal_latch_map_1(&mut self) -> ZqCalLatchMap1W<DenaliCtl190Spec> {
        ZqCalLatchMap1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Encoded number of banks on the DRAM(s)."]
    #[inline(always)]
    #[must_use]
    pub fn bank_diff(&mut self) -> BankDiffW<DenaliCtl190Spec> {
        BankDiffW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Difference between number of address pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn row_diff(&mut self) -> RowDiffW<DenaliCtl190Spec> {
        RowDiffW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_190::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_190::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl190Spec;
impl crate::RegisterSpec for DenaliCtl190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_190::R`](R) reader structure"]
impl crate::Readable for DenaliCtl190Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_190::W`](W) writer structure"]
impl crate::Writable for DenaliCtl190Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_190 to value 0x0202"]
impl crate::Resettable for DenaliCtl190Spec {
    const RESET_VALUE: u32 = 0x0202;
}
