#[doc = "Register `PMUCRU_CLKGATE_CON2` reader"]
pub type R = crate::R<PmucruClkgateCon2Spec>;
#[doc = "Register `PMUCRU_CLKGATE_CON2` writer"]
pub type W = crate::W<PmucruClkgateCon2Spec>;
#[doc = "Field `FCLK_CM0S_EN` reader - fclk_cm0s clock disable bit When HIGH, disable clock"]
pub type FclkCm0sEnR = crate::BitReader;
#[doc = "Field `FCLK_CM0S_EN` writer - fclk_cm0s clock disable bit When HIGH, disable clock"]
pub type FclkCm0sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_CM0S_EN` reader - sclk_cm0s clock disable bit When HIGH, disable clock"]
pub type SclkCm0sEnR = crate::BitReader;
#[doc = "Field `SCLK_CM0S_EN` writer - sclk_cm0s clock disable bit When HIGH, disable clock"]
pub type SclkCm0sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_CM0S_EN` reader - hclk_cm0s clock disable bit When HIGH, disable clock"]
pub type HclkCm0sEnR = crate::BitReader;
#[doc = "Field `HCLK_CM0S_EN` writer - hclk_cm0s clock disable bit When HIGH, disable clock"]
pub type HclkCm0sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCLK_CM0S_EN` reader - dclk_cm0s clock disable bit When HIGH, disable clock"]
pub type DclkCm0sEnR = crate::BitReader;
#[doc = "Field `DCLK_CM0S_EN` writer - dclk_cm0s clock disable bit When HIGH, disable clock"]
pub type DclkCm0sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_NOC_PMU_EN` reader - hclk_noc_pmu clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkNocPmuEnR = crate::BitReader;
#[doc = "Field `HCLK_NOC_PMU_EN` writer - hclk_noc_pmu clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkNocPmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - fclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn fclk_cm0s_en(&self) -> FclkCm0sEnR {
        FclkCm0sEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn sclk_cm0s_en(&self) -> SclkCm0sEnR {
        SclkCm0sEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_cm0s_en(&self) -> HclkCm0sEnR {
        HclkCm0sEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - dclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn dclk_cm0s_en(&self) -> DclkCm0sEnR {
        DclkCm0sEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - hclk_noc_pmu clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_noc_pmu_en(&self) -> HclkNocPmuEnR {
        HclkNocPmuEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - fclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn fclk_cm0s_en(&mut self) -> FclkCm0sEnW<PmucruClkgateCon2Spec> {
        FclkCm0sEnW::new(self, 0)
    }
    #[doc = "Bit 1 - sclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_cm0s_en(&mut self) -> SclkCm0sEnW<PmucruClkgateCon2Spec> {
        SclkCm0sEnW::new(self, 1)
    }
    #[doc = "Bit 2 - hclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_cm0s_en(&mut self) -> HclkCm0sEnW<PmucruClkgateCon2Spec> {
        HclkCm0sEnW::new(self, 2)
    }
    #[doc = "Bit 3 - dclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_cm0s_en(&mut self) -> DclkCm0sEnW<PmucruClkgateCon2Spec> {
        DclkCm0sEnW::new(self, 3)
    }
    #[doc = "Bit 5 - hclk_noc_pmu clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_noc_pmu_en(&mut self) -> HclkNocPmuEnW<PmucruClkgateCon2Spec> {
        HclkNocPmuEnW::new(self, 5)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkgateCon2Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkgate_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkgate_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkgateCon2Spec;
impl crate::RegisterSpec for PmucruClkgateCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clkgate_con2::R`](R) reader structure"]
impl crate::Readable for PmucruClkgateCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clkgate_con2::W`](W) writer structure"]
impl crate::Writable for PmucruClkgateCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKGATE_CON2 to value 0"]
impl crate::Resettable for PmucruClkgateCon2Spec {
    const RESET_VALUE: u32 = 0;
}
