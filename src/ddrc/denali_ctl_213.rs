#[doc = "Register `DENALI_CTL_213` reader"]
pub type R = crate::R<DenaliCtl213Spec>;
#[doc = "Register `DENALI_CTL_213` writer"]
pub type W = crate::W<DenaliCtl213Spec>;
#[doc = "Field `TODTH_WR` reader - Defines the DRAM minimum ODT high time after an ODT assertion for a write command."]
pub type TodthWrR = crate::FieldReader;
#[doc = "Field `TODTH_WR` writer - Defines the DRAM minimum ODT high time after an ODT assertion for a write command."]
pub type TodthWrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TODTH_RD` reader - Defines the DRAM minimum ODT high time after an ODT assertion for a read command."]
pub type TodthRdR = crate::FieldReader;
#[doc = "Field `TODTH_RD` writer - Defines the DRAM minimum ODT high time after an ODT assertion for a read command."]
pub type TodthRdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ODT_EN_F0` reader - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
pub type OdtEnF0R = crate::BitReader;
#[doc = "Field `ODT_EN_F0` writer - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
pub type OdtEnF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT_EN_F1` reader - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
pub type OdtEnF1R = crate::BitReader;
#[doc = "Field `ODT_EN_F1` writer - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
pub type OdtEnF1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Defines the DRAM minimum ODT high time after an ODT assertion for a write command."]
    #[inline(always)]
    pub fn todth_wr(&self) -> TodthWrR {
        TodthWrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the DRAM minimum ODT high time after an ODT assertion for a read command."]
    #[inline(always)]
    pub fn todth_rd(&self) -> TodthRdR {
        TodthRdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
    #[inline(always)]
    pub fn odt_en_f0(&self) -> OdtEnF0R {
        OdtEnF0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
    #[inline(always)]
    pub fn odt_en_f1(&self) -> OdtEnF1R {
        OdtEnF1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DRAM minimum ODT high time after an ODT assertion for a write command."]
    #[inline(always)]
    #[must_use]
    pub fn todth_wr(&mut self) -> TodthWrW<DenaliCtl213Spec> {
        TodthWrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the DRAM minimum ODT high time after an ODT assertion for a read command."]
    #[inline(always)]
    #[must_use]
    pub fn todth_rd(&mut self) -> TodthRdW<DenaliCtl213Spec> {
        TodthRdW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
    #[inline(always)]
    #[must_use]
    pub fn odt_en_f0(&mut self) -> OdtEnF0W<DenaliCtl213Spec> {
        OdtEnF0W::new(self, 16)
    }
    #[doc = "Bit 24 - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
    #[inline(always)]
    #[must_use]
    pub fn odt_en_f1(&mut self) -> OdtEnF1W<DenaliCtl213Spec> {
        OdtEnF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_213::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_213::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl213Spec;
impl crate::RegisterSpec for DenaliCtl213Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_213::R`](R) reader structure"]
impl crate::Readable for DenaliCtl213Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_213::W`](W) writer structure"]
impl crate::Writable for DenaliCtl213Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_213 to value 0"]
impl crate::Resettable for DenaliCtl213Spec {
    const RESET_VALUE: u32 = 0;
}
