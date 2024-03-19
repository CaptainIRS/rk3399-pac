#[doc = "Register `DP_LN1_LINK_TRAINING_CTL` reader"]
pub type R = crate::R<DpLn1LinkTrainingCtlSpec>;
#[doc = "Register `DP_LN1_LINK_TRAINING_CTL` writer"]
pub type W = crate::W<DpLn1LinkTrainingCtlSpec>;
#[doc = "Lane 1 output amplitude setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DriveCurrentSet1 {
    #[doc = "3: 1200 mV,"]
    B11 = 3,
    #[doc = "2: 800 mV,"]
    B10 = 2,
    #[doc = "1: 600 mV,"]
    B01 = 1,
    #[doc = "0: 400 mV. This bit's type is R/W."]
    B00 = 0,
}
impl From<DriveCurrentSet1> for u8 {
    #[inline(always)]
    fn from(variant: DriveCurrentSet1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DriveCurrentSet1 {
    type Ux = u8;
}
#[doc = "Field `DRIVE_CURRENT_SET_1` reader - Lane 1 output amplitude setting"]
pub type DriveCurrentSet1R = crate::FieldReader<DriveCurrentSet1>;
impl DriveCurrentSet1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DriveCurrentSet1 {
        match self.bits {
            3 => DriveCurrentSet1::B11,
            2 => DriveCurrentSet1::B10,
            1 => DriveCurrentSet1::B01,
            0 => DriveCurrentSet1::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "1200 mV,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DriveCurrentSet1::B11
    }
    #[doc = "800 mV,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DriveCurrentSet1::B10
    }
    #[doc = "600 mV,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DriveCurrentSet1::B01
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DriveCurrentSet1::B00
    }
}
#[doc = "Field `DRIVE_CURRENT_SET_1` writer - Lane 1 output amplitude setting"]
pub type DriveCurrentSet1W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DriveCurrentSet1>;
impl<'a, REG> DriveCurrentSet1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1200 mV,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet1::B11)
    }
    #[doc = "800 mV,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet1::B10)
    }
    #[doc = "600 mV,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet1::B01)
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet1::B00)
    }
}
#[doc = "Field `MAX_DRIVE_REACH_1` reader - This bit field is set to 1 automatically when \n\nmax driving current level of DP Tx is \n\nreached. For more information, refer to \n\nMAX_PRE_REACH_1. For test purpose only. \n\nThis bit's type is RO."]
pub type MaxDriveReach1R = crate::BitReader;
#[doc = "Lane 1 pre-emphasis level setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PreEmphasisSet1 {
    #[doc = "3: 9.5 dB,"]
    B11 = 3,
    #[doc = "2: 6.0 dB,"]
    B10 = 2,
    #[doc = "1: 3.5 dB,"]
    B01 = 1,
    #[doc = "0: 0 dB (No pre-emphasis). This bit's type is R/W."]
    B00 = 0,
}
impl From<PreEmphasisSet1> for u8 {
    #[inline(always)]
    fn from(variant: PreEmphasisSet1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PreEmphasisSet1 {
    type Ux = u8;
}
#[doc = "Field `PRE_EMPHASIS_SET_1` reader - Lane 1 pre-emphasis level setting"]
pub type PreEmphasisSet1R = crate::FieldReader<PreEmphasisSet1>;
impl PreEmphasisSet1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PreEmphasisSet1 {
        match self.bits {
            3 => PreEmphasisSet1::B11,
            2 => PreEmphasisSet1::B10,
            1 => PreEmphasisSet1::B01,
            0 => PreEmphasisSet1::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "9.5 dB,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == PreEmphasisSet1::B11
    }
    #[doc = "6.0 dB,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == PreEmphasisSet1::B10
    }
    #[doc = "3.5 dB,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == PreEmphasisSet1::B01
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == PreEmphasisSet1::B00
    }
}
#[doc = "Field `PRE_EMPHASIS_SET_1` writer - Lane 1 pre-emphasis level setting"]
pub type PreEmphasisSet1W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PreEmphasisSet1>;
impl<'a, REG> PreEmphasisSet1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "9.5 dB,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet1::B11)
    }
    #[doc = "6.0 dB,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet1::B10)
    }
    #[doc = "3.5 dB,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet1::B01)
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet1::B00)
    }
}
#[doc = "Field `MAX_PRE_REACH_1` reader - This bit field is set to 1 automatically when \n\nmax pre-emphasis level of DP Tx is \n\nreached. \n\nNote that the MAX_PRE_REACH_1 and \n\nMAX_DRIVE_REACH_1 have the same \n\nvalue like the following table. \n\nBoth of MAX_PRE_REACH_1 and \n\nMAX_DRIVE_REACH_1 are for test purpose \n\nonly. \n\nThis bit's type is RO."]
pub type MaxPreReach1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Lane 1 output amplitude setting"]
    #[inline(always)]
    pub fn drive_current_set_1(&self) -> DriveCurrentSet1R {
        DriveCurrentSet1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This bit field is set to 1 automatically when \n\nmax driving current level of DP Tx is \n\nreached. For more information, refer to \n\nMAX_PRE_REACH_1. For test purpose only. \n\nThis bit's type is RO."]
    #[inline(always)]
    pub fn max_drive_reach_1(&self) -> MaxDriveReach1R {
        MaxDriveReach1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Lane 1 pre-emphasis level setting"]
    #[inline(always)]
    pub fn pre_emphasis_set_1(&self) -> PreEmphasisSet1R {
        PreEmphasisSet1R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - This bit field is set to 1 automatically when \n\nmax pre-emphasis level of DP Tx is \n\nreached. \n\nNote that the MAX_PRE_REACH_1 and \n\nMAX_DRIVE_REACH_1 have the same \n\nvalue like the following table. \n\nBoth of MAX_PRE_REACH_1 and \n\nMAX_DRIVE_REACH_1 are for test purpose \n\nonly. \n\nThis bit's type is RO."]
    #[inline(always)]
    pub fn max_pre_reach_1(&self) -> MaxPreReach1R {
        MaxPreReach1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lane 1 output amplitude setting"]
    #[inline(always)]
    #[must_use]
    pub fn drive_current_set_1(&mut self) -> DriveCurrentSet1W<DpLn1LinkTrainingCtlSpec> {
        DriveCurrentSet1W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Lane 1 pre-emphasis level setting"]
    #[inline(always)]
    #[must_use]
    pub fn pre_emphasis_set_1(&mut self) -> PreEmphasisSet1W<DpLn1LinkTrainingCtlSpec> {
        PreEmphasisSet1W::new(self, 3)
    }
}
#[doc = "DP Lane 1 Link Training Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_ln1_link_training_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_ln1_link_training_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpLn1LinkTrainingCtlSpec;
impl crate::RegisterSpec for DpLn1LinkTrainingCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_ln1_link_training_ctl::R`](R) reader structure"]
impl crate::Readable for DpLn1LinkTrainingCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_ln1_link_training_ctl::W`](W) writer structure"]
impl crate::Writable for DpLn1LinkTrainingCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets DP_LN1_LINK_TRAINING_CTL to value 0"]
impl crate::Resettable for DpLn1LinkTrainingCtlSpec {
    const RESET_VALUE: u32 = 0;
}
