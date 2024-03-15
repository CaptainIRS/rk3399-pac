#[doc = "Register `PMU_DDR_SREF_ST` reader"]
pub type R = crate::R<PmuDdrSrefStSpec>;
#[doc = "Register `PMU_DDR_SREF_ST` writer"]
pub type W = crate::W<PmuDdrSrefStSpec>;
#[doc = "Field `DDRC0_SREF_DONE_EXT` reader - ddr controller 0 self re-fresh done, active high"]
pub type Ddrc0SrefDoneExtR = crate::BitReader;
#[doc = "Field `DDRC0_SREF_DONE_EXT` writer - ddr controller 0 self re-fresh done, active high"]
pub type Ddrc0SrefDoneExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRC1_SREF_DONE_EXT` reader - ddr controller 1 self re-fresh done, active high"]
pub type Ddrc1SrefDoneExtR = crate::BitReader;
#[doc = "Field `DDRC1_SREF_DONE_EXT` writer - ddr controller 1 self re-fresh done, active high"]
pub type Ddrc1SrefDoneExtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ddr controller 0 self re-fresh done, active high"]
    #[inline(always)]
    pub fn ddrc0_sref_done_ext(&self) -> Ddrc0SrefDoneExtR {
        Ddrc0SrefDoneExtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - ddr controller 1 self re-fresh done, active high"]
    #[inline(always)]
    pub fn ddrc1_sref_done_ext(&self) -> Ddrc1SrefDoneExtR {
        Ddrc1SrefDoneExtR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ddr controller 0 self re-fresh done, active high"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc0_sref_done_ext(&mut self) -> Ddrc0SrefDoneExtW<PmuDdrSrefStSpec> {
        Ddrc0SrefDoneExtW::new(self, 0)
    }
    #[doc = "Bit 2 - ddr controller 1 self re-fresh done, active high"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc1_sref_done_ext(&mut self) -> Ddrc1SrefDoneExtW<PmuDdrSrefStSpec> {
        Ddrc1SrefDoneExtW::new(self, 2)
    }
}
#[doc = "pmu ddr self refresh status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_ddr_sref_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_ddr_sref_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuDdrSrefStSpec;
impl crate::RegisterSpec for PmuDdrSrefStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_ddr_sref_st::R`](R) reader structure"]
impl crate::Readable for PmuDdrSrefStSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_ddr_sref_st::W`](W) writer structure"]
impl crate::Writable for PmuDdrSrefStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_DDR_SREF_ST to value 0"]
impl crate::Resettable for PmuDdrSrefStSpec {
    const RESET_VALUE: u32 = 0;
}
