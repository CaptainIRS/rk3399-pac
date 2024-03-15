#[doc = "Register `DP_HW_LINK_TRAINING_CTL` reader"]
pub type R = crate::R<DpHwLinkTrainingCtlSpec>;
#[doc = "Register `DP_HW_LINK_TRAINING_CTL` writer"]
pub type W = crate::W<DpHwLinkTrainingCtlSpec>;
#[doc = "Field `HW_TRAINING_EN` reader - Link training sequence enable Write 1 to enable training sequence, write 0 to force training sequence stop, this bit will self-clear when training done. This bit's type is R/W. This bit is self cleared."]
pub type HwTrainingEnR = crate::BitReader;
#[doc = "Field `HW_TRAINING_EN` writer - Link training sequence enable Write 1 to enable training sequence, write 0 to force training sequence stop, this bit will self-clear when training done. This bit's type is R/W. This bit is self cleared."]
pub type HwTrainingEnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Training error code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HwTrainingErrorCode {
    #[doc = "0: EQ_LOOP_5_TIME This bit's type is RO."]
    D0 = 0,
    #[doc = "1: EQ_LOOP_5_TIME This bit's type is RO."]
    D1 = 1,
    #[doc = "2: EQ_LOOP_5_TIME This bit's type is RO."]
    D2 = 2,
    #[doc = "3: EQ_LOOP_5_TIME This bit's type is RO."]
    D3 = 3,
    #[doc = "4: EQ_LOOP_5_TIME This bit's type is RO."]
    D4 = 4,
    #[doc = "5: EQ_LOOP_5_TIME This bit's type is RO."]
    D5 = 5,
    #[doc = "6: EQ_LOOP_5_TIME This bit's type is RO."]
    D6 = 6,
}
impl From<HwTrainingErrorCode> for u8 {
    #[inline(always)]
    fn from(variant: HwTrainingErrorCode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HwTrainingErrorCode {
    type Ux = u8;
}
#[doc = "Field `HW_TRAINING_ERROR_CODE` reader - Training error code"]
pub type HwTrainingErrorCodeR = crate::FieldReader<HwTrainingErrorCode>;
impl HwTrainingErrorCodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HwTrainingErrorCode> {
        match self.bits {
            0 => Some(HwTrainingErrorCode::D0),
            1 => Some(HwTrainingErrorCode::D1),
            2 => Some(HwTrainingErrorCode::D2),
            3 => Some(HwTrainingErrorCode::D3),
            4 => Some(HwTrainingErrorCode::D4),
            5 => Some(HwTrainingErrorCode::D5),
            6 => Some(HwTrainingErrorCode::D6),
            _ => None,
        }
    }
    #[doc = "EQ_LOOP_5_TIME This bit's type is RO."]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == HwTrainingErrorCode::D0
    }
    #[doc = "EQ_LOOP_5_TIME This bit's type is RO."]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == HwTrainingErrorCode::D1
    }
    #[doc = "EQ_LOOP_5_TIME This bit's type is RO."]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == HwTrainingErrorCode::D2
    }
    #[doc = "EQ_LOOP_5_TIME This bit's type is RO."]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == HwTrainingErrorCode::D3
    }
    #[doc = "EQ_LOOP_5_TIME This bit's type is RO."]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == HwTrainingErrorCode::D4
    }
    #[doc = "EQ_LOOP_5_TIME This bit's type is RO."]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == HwTrainingErrorCode::D5
    }
    #[doc = "EQ_LOOP_5_TIME This bit's type is RO."]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == HwTrainingErrorCode::D6
    }
}
impl R {
    #[doc = "Bit 0 - Link training sequence enable Write 1 to enable training sequence, write 0 to force training sequence stop, this bit will self-clear when training done. This bit's type is R/W. This bit is self cleared."]
    #[inline(always)]
    pub fn hw_training_en(&self) -> HwTrainingEnR {
        HwTrainingEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Training error code"]
    #[inline(always)]
    pub fn hw_training_error_code(&self) -> HwTrainingErrorCodeR {
        HwTrainingErrorCodeR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Link training sequence enable Write 1 to enable training sequence, write 0 to force training sequence stop, this bit will self-clear when training done. This bit's type is R/W. This bit is self cleared."]
    #[inline(always)]
    #[must_use]
    pub fn hw_training_en(&mut self) -> HwTrainingEnW<DpHwLinkTrainingCtlSpec> {
        HwTrainingEnW::new(self, 0)
    }
}
#[doc = "DP HW LINK TRAINING_CONTROL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_hw_link_training_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_hw_link_training_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpHwLinkTrainingCtlSpec;
impl crate::RegisterSpec for DpHwLinkTrainingCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_hw_link_training_ctl::R`](R) reader structure"]
impl crate::Readable for DpHwLinkTrainingCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_hw_link_training_ctl::W`](W) writer structure"]
impl crate::Writable for DpHwLinkTrainingCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets DP_HW_LINK_TRAINING_CTL to value 0"]
impl crate::Resettable for DpHwLinkTrainingCtlSpec {
    const RESET_VALUE: u32 = 0;
}
