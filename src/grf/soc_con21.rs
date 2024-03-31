#[doc = "Register `SOC_CON21` reader"]
pub type R = crate::R<SocCon21Spec>;
#[doc = "Register `SOC_CON21` writer"]
pub type W = crate::W<SocCon21Spec>;
#[doc = "Field `DPHY_RX0_ENABLE` reader - dphy_rx0_enable bit control"]
pub type DphyRx0EnableR = crate::FieldReader;
#[doc = "Field `DPHY_RX0_ENABLE` writer - dphy_rx0_enable bit control"]
pub type DphyRx0EnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_RX0_FORCERXMODE` reader - dphy_rx0_forcerxmode bit control"]
pub type DphyRx0ForcerxmodeR = crate::FieldReader;
#[doc = "Field `DPHY_RX0_FORCERXMODE` writer - dphy_rx0_forcerxmode bit control"]
pub type DphyRx0ForcerxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_RX0_FORCETXSTOPMODE` reader - dphy_rx0_forcetxstopmode bit control"]
pub type DphyRx0ForcetxstopmodeR = crate::FieldReader;
#[doc = "Field `DPHY_RX0_FORCETXSTOPMODE` writer - dphy_rx0_forcetxstopmode bit control"]
pub type DphyRx0ForcetxstopmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_RX0_TURNDISABLE` reader - dphy_rx0_turndisable bit control"]
pub type DphyRx0TurndisableR = crate::FieldReader;
#[doc = "Field `DPHY_RX0_TURNDISABLE` writer - dphy_rx0_turndisable bit control"]
pub type DphyRx0TurndisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - dphy_rx0_enable bit control"]
    #[inline(always)]
    pub fn dphy_rx0_enable(&self) -> DphyRx0EnableR {
        DphyRx0EnableR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - dphy_rx0_forcerxmode bit control"]
    #[inline(always)]
    pub fn dphy_rx0_forcerxmode(&self) -> DphyRx0ForcerxmodeR {
        DphyRx0ForcerxmodeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - dphy_rx0_forcetxstopmode bit control"]
    #[inline(always)]
    pub fn dphy_rx0_forcetxstopmode(&self) -> DphyRx0ForcetxstopmodeR {
        DphyRx0ForcetxstopmodeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - dphy_rx0_turndisable bit control"]
    #[inline(always)]
    pub fn dphy_rx0_turndisable(&self) -> DphyRx0TurndisableR {
        DphyRx0TurndisableR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - dphy_rx0_enable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_enable(&mut self) -> DphyRx0EnableW<SocCon21Spec> {
        DphyRx0EnableW::new(self, 0)
    }
    #[doc = "Bits 4:7 - dphy_rx0_forcerxmode bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_forcerxmode(&mut self) -> DphyRx0ForcerxmodeW<SocCon21Spec> {
        DphyRx0ForcerxmodeW::new(self, 4)
    }
    #[doc = "Bits 8:11 - dphy_rx0_forcetxstopmode bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_forcetxstopmode(&mut self) -> DphyRx0ForcetxstopmodeW<SocCon21Spec> {
        DphyRx0ForcetxstopmodeW::new(self, 8)
    }
    #[doc = "Bits 12:15 - dphy_rx0_turndisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_turndisable(&mut self) -> DphyRx0TurndisableW<SocCon21Spec> {
        DphyRx0TurndisableW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SocCon21Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocCon21Spec;
impl crate::RegisterSpec for SocCon21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_con21::R`](R) reader structure"]
impl crate::Readable for SocCon21Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_con21::W`](W) writer structure"]
impl crate::Writable for SocCon21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CON21 to value 0x02cb"]
impl crate::Resettable for SocCon21Spec {
    const RESET_VALUE: u32 = 0x02cb;
}
