#[doc = "Register `CABC_CTRL2` reader"]
pub type R = crate::R<CabcCtrl2Spec>;
#[doc = "Register `CABC_CTRL2` writer"]
pub type W = crate::W<CabcCtrl2Spec>;
#[doc = "Field `CABC_STAGE_DOWN` reader - when mul mode ,scale stage down (0.667~1 * 256).\n\nwhen add mode ,scale stage down (0x00~0xff)."]
pub type CabcStageDownR = crate::FieldReader;
#[doc = "Field `CABC_STAGE_DOWN` writer - when mul mode ,scale stage down (0.667~1 * 256).\n\nwhen add mode ,scale stage down (0x00~0xff)."]
pub type CabcStageDownW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CABC_STAGE_UP` reader - when mul mode ,scale stage up (1~1.5 * 256).\n\nwhen add mode ,scale stage up (0x00~0xff)."]
pub type CabcStageUpR = crate::FieldReader<u16>;
#[doc = "Field `CABC_STAGE_UP` writer - when mul mode ,scale stage up (1~1.5 * 256).\n\nwhen add mode ,scale stage up (0x00~0xff)."]
pub type CabcStageUpW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CabcStageUpMode {
    #[doc = "0: mul mode"]
    B0 = 0,
    #[doc = "1: add mode"]
    B1 = 1,
}
impl From<CabcStageUpMode> for bool {
    #[inline(always)]
    fn from(variant: CabcStageUpMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CABC_STAGE_UP_MODE` reader - "]
pub type CabcStageUpModeR = crate::BitReader<CabcStageUpMode>;
impl CabcStageUpModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CabcStageUpMode {
        match self.bits {
            false => CabcStageUpMode::B0,
            true => CabcStageUpMode::B1,
        }
    }
    #[doc = "mul mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CabcStageUpMode::B0
    }
    #[doc = "add mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CabcStageUpMode::B1
    }
}
#[doc = "Field `CABC_STAGE_UP_MODE` writer - "]
pub type CabcStageUpModeW<'a, REG> = crate::BitWriter<'a, REG, CabcStageUpMode>;
impl<'a, REG> CabcStageUpModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "mul mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CabcStageUpMode::B0)
    }
    #[doc = "add mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CabcStageUpMode::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - when mul mode ,scale stage down (0.667~1 * 256).\n\nwhen add mode ,scale stage down (0x00~0xff)."]
    #[inline(always)]
    pub fn cabc_stage_down(&self) -> CabcStageDownR {
        CabcStageDownR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - when mul mode ,scale stage up (1~1.5 * 256).\n\nwhen add mode ,scale stage up (0x00~0xff)."]
    #[inline(always)]
    pub fn cabc_stage_up(&self) -> CabcStageUpR {
        CabcStageUpR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cabc_stage_up_mode(&self) -> CabcStageUpModeR {
        CabcStageUpModeR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - when mul mode ,scale stage down (0.667~1 * 256).\n\nwhen add mode ,scale stage down (0x00~0xff)."]
    #[inline(always)]
    #[must_use]
    pub fn cabc_stage_down(&mut self) -> CabcStageDownW<CabcCtrl2Spec> {
        CabcStageDownW::new(self, 0)
    }
    #[doc = "Bits 8:16 - when mul mode ,scale stage up (1~1.5 * 256).\n\nwhen add mode ,scale stage up (0x00~0xff)."]
    #[inline(always)]
    #[must_use]
    pub fn cabc_stage_up(&mut self) -> CabcStageUpW<CabcCtrl2Spec> {
        CabcStageUpW::new(self, 8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cabc_stage_up_mode(&mut self) -> CabcStageUpModeW<CabcCtrl2Spec> {
        CabcStageUpModeW::new(self, 19)
    }
}
#[doc = "Content Adaptive Backlight Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcCtrl2Spec;
impl crate::RegisterSpec for CabcCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_ctrl2::R`](R) reader structure"]
impl crate::Readable for CabcCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cabc_ctrl2::W`](W) writer structure"]
impl crate::Writable for CabcCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_CTRL2 to value 0x0001_10f0"]
impl crate::Resettable for CabcCtrl2Spec {
    const RESET_VALUE: u32 = 0x0001_10f0;
}
