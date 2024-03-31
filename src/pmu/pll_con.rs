#[doc = "Register `PLL_CON` reader"]
pub type R = crate::R<PllConSpec>;
#[doc = "Register `PLL_CON` writer"]
pub type W = crate::W<PllConSpec>;
#[doc = "Field `PLL_PD_CFG` reader - pll power down configured by hardware"]
pub type PllPdCfgR = crate::FieldReader;
#[doc = "Field `PLL_PD_CFG` writer - pll power down configured by hardware"]
pub type PllPdCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SFT_PLL_PD` reader - pll power down configured by sftware."]
pub type SftPllPdR = crate::FieldReader;
#[doc = "Field `SFT_PLL_PD` writer - pll power down configured by sftware."]
pub type SftPllPdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - pll power down configured by hardware"]
    #[inline(always)]
    pub fn pll_pd_cfg(&self) -> PllPdCfgR {
        PllPdCfgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - pll power down configured by sftware."]
    #[inline(always)]
    pub fn sft_pll_pd(&self) -> SftPllPdR {
        SftPllPdR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - pll power down configured by hardware"]
    #[inline(always)]
    #[must_use]
    pub fn pll_pd_cfg(&mut self) -> PllPdCfgW<PllConSpec> {
        PllPdCfgW::new(self, 0)
    }
    #[doc = "Bits 8:15 - pll power down configured by sftware."]
    #[inline(always)]
    #[must_use]
    pub fn sft_pll_pd(&mut self) -> SftPllPdW<PllConSpec> {
        SftPllPdW::new(self, 8)
    }
}
#[doc = "PLL low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllConSpec;
impl crate::RegisterSpec for PllConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_con::R`](R) reader structure"]
impl crate::Readable for PllConSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_con::W`](W) writer structure"]
impl crate::Writable for PllConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_CON to value 0"]
impl crate::Resettable for PllConSpec {
    const RESET_VALUE: u32 = 0;
}
