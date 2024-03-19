#[doc = "Register `GRF_SOC_CON23` reader"]
pub type R = crate::R<GrfSocCon23Spec>;
#[doc = "Register `GRF_SOC_CON23` writer"]
pub type W = crate::W<GrfSocCon23Spec>;
#[doc = "Field `DPHY_TX1RX1_ENABLE` reader - dphy_tx1rx1_enable bit control"]
pub type DphyTx1rx1EnableR = crate::FieldReader;
#[doc = "Field `DPHY_TX1RX1_ENABLE` writer - dphy_tx1rx1_enable bit control"]
pub type DphyTx1rx1EnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_TX1RX1_FORCERXMODE` reader - dphy_tx1rx1_forcerxmode bit control"]
pub type DphyTx1rx1ForcerxmodeR = crate::FieldReader;
#[doc = "Field `DPHY_TX1RX1_FORCERXMODE` writer - dphy_tx1rx1_forcerxmode bit control"]
pub type DphyTx1rx1ForcerxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_TX1RX1_FORCETXSTOPMODE` reader - dphy_tx1rx1_forcetxstopmode bit control"]
pub type DphyTx1rx1ForcetxstopmodeR = crate::FieldReader;
#[doc = "Field `DPHY_TX1RX1_FORCETXSTOPMODE` writer - dphy_tx1rx1_forcetxstopmode bit control"]
pub type DphyTx1rx1ForcetxstopmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_TX1RX1_TURNDISABLE` reader - dphy_tx1rx1_turndisable bit control"]
pub type DphyTx1rx1TurndisableR = crate::FieldReader;
#[doc = "Field `DPHY_TX1RX1_TURNDISABLE` writer - dphy_tx1rx1_turndisable bit control"]
pub type DphyTx1rx1TurndisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - dphy_tx1rx1_enable bit control"]
    #[inline(always)]
    pub fn dphy_tx1rx1_enable(&self) -> DphyTx1rx1EnableR {
        DphyTx1rx1EnableR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - dphy_tx1rx1_forcerxmode bit control"]
    #[inline(always)]
    pub fn dphy_tx1rx1_forcerxmode(&self) -> DphyTx1rx1ForcerxmodeR {
        DphyTx1rx1ForcerxmodeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - dphy_tx1rx1_forcetxstopmode bit control"]
    #[inline(always)]
    pub fn dphy_tx1rx1_forcetxstopmode(&self) -> DphyTx1rx1ForcetxstopmodeR {
        DphyTx1rx1ForcetxstopmodeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - dphy_tx1rx1_turndisable bit control"]
    #[inline(always)]
    pub fn dphy_tx1rx1_turndisable(&self) -> DphyTx1rx1TurndisableR {
        DphyTx1rx1TurndisableR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - dphy_tx1rx1_enable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1rx1_enable(&mut self) -> DphyTx1rx1EnableW<GrfSocCon23Spec> {
        DphyTx1rx1EnableW::new(self, 0)
    }
    #[doc = "Bits 4:7 - dphy_tx1rx1_forcerxmode bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1rx1_forcerxmode(&mut self) -> DphyTx1rx1ForcerxmodeW<GrfSocCon23Spec> {
        DphyTx1rx1ForcerxmodeW::new(self, 4)
    }
    #[doc = "Bits 8:11 - dphy_tx1rx1_forcetxstopmode bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1rx1_forcetxstopmode(&mut self) -> DphyTx1rx1ForcetxstopmodeW<GrfSocCon23Spec> {
        DphyTx1rx1ForcetxstopmodeW::new(self, 8)
    }
    #[doc = "Bits 12:15 - dphy_tx1rx1_turndisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1rx1_turndisable(&mut self) -> DphyTx1rx1TurndisableW<GrfSocCon23Spec> {
        DphyTx1rx1TurndisableW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon23Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon23Spec;
impl crate::RegisterSpec for GrfSocCon23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con23::R`](R) reader structure"]
impl crate::Readable for GrfSocCon23Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con23::W`](W) writer structure"]
impl crate::Writable for GrfSocCon23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON23 to value 0x21"]
impl crate::Resettable for GrfSocCon23Spec {
    const RESET_VALUE: u32 = 0x21;
}
