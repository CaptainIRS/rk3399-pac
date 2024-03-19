#[doc = "Register `GRF_TSADC_TESTBIT_L` reader"]
pub type R = crate::R<GrfTsadcTestbitLSpec>;
#[doc = "Register `GRF_TSADC_TESTBIT_L` writer"]
pub type W = crate::W<GrfTsadcTestbitLSpec>;
#[doc = "Field `GRF_TSADC_TSEN_PD_0` reader - grf_tsadc_tsen_pd_0"]
pub type GrfTsadcTsenPd0R = crate::BitReader;
#[doc = "Field `GRF_TSADC_TSEN_PD_0` writer - grf_tsadc_tsen_pd_0"]
pub type GrfTsadcTsenPd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRF_TSADC_TSEN_PD_1` reader - grf_tsadc_tsen_pd_1"]
pub type GrfTsadcTsenPd1R = crate::BitReader;
#[doc = "Field `GRF_TSADC_TSEN_PD_1` writer - grf_tsadc_tsen_pd_1"]
pub type GrfTsadcTsenPd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRF_TSADC_CLK_SEL` reader - grf_tsadc_clk_sel"]
pub type GrfTsadcClkSelR = crate::BitReader;
#[doc = "Field `GRF_TSADC_CLK_SEL` writer - grf_tsadc_clk_sel"]
pub type GrfTsadcClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRF_TSADC_DIG_BYPASS` reader - grf_tsadc_dig_bypass"]
pub type GrfTsadcDigBypassR = crate::BitReader;
#[doc = "Field `GRF_TSADC_DIG_BYPASS` writer - grf_tsadc_dig_bypass"]
pub type GrfTsadcDigBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - grf_tsadc_tsen_pd_0"]
    #[inline(always)]
    pub fn grf_tsadc_tsen_pd_0(&self) -> GrfTsadcTsenPd0R {
        GrfTsadcTsenPd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - grf_tsadc_tsen_pd_1"]
    #[inline(always)]
    pub fn grf_tsadc_tsen_pd_1(&self) -> GrfTsadcTsenPd1R {
        GrfTsadcTsenPd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - grf_tsadc_clk_sel"]
    #[inline(always)]
    pub fn grf_tsadc_clk_sel(&self) -> GrfTsadcClkSelR {
        GrfTsadcClkSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - grf_tsadc_dig_bypass"]
    #[inline(always)]
    pub fn grf_tsadc_dig_bypass(&self) -> GrfTsadcDigBypassR {
        GrfTsadcDigBypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - grf_tsadc_tsen_pd_0"]
    #[inline(always)]
    #[must_use]
    pub fn grf_tsadc_tsen_pd_0(&mut self) -> GrfTsadcTsenPd0W<GrfTsadcTestbitLSpec> {
        GrfTsadcTsenPd0W::new(self, 0)
    }
    #[doc = "Bit 1 - grf_tsadc_tsen_pd_1"]
    #[inline(always)]
    #[must_use]
    pub fn grf_tsadc_tsen_pd_1(&mut self) -> GrfTsadcTsenPd1W<GrfTsadcTestbitLSpec> {
        GrfTsadcTsenPd1W::new(self, 1)
    }
    #[doc = "Bit 2 - grf_tsadc_clk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn grf_tsadc_clk_sel(&mut self) -> GrfTsadcClkSelW<GrfTsadcTestbitLSpec> {
        GrfTsadcClkSelW::new(self, 2)
    }
    #[doc = "Bit 3 - grf_tsadc_dig_bypass"]
    #[inline(always)]
    #[must_use]
    pub fn grf_tsadc_dig_bypass(&mut self) -> GrfTsadcDigBypassW<GrfTsadcTestbitLSpec> {
        GrfTsadcDigBypassW::new(self, 3)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfTsadcTestbitLSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "saradc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_tsadc_testbit_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_tsadc_testbit_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfTsadcTestbitLSpec;
impl crate::RegisterSpec for GrfTsadcTestbitLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_tsadc_testbit_l::R`](R) reader structure"]
impl crate::Readable for GrfTsadcTestbitLSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_tsadc_testbit_l::W`](W) writer structure"]
impl crate::Writable for GrfTsadcTestbitLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_TSADC_TESTBIT_L to value 0"]
impl crate::Resettable for GrfTsadcTestbitLSpec {
    const RESET_VALUE: u32 = 0;
}
