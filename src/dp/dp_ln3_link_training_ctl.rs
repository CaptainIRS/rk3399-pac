#[doc = "Register `DP_LN3_LINK_TRAINING_CTL` reader"]
pub type R = crate::R<DpLn3LinkTrainingCtlSpec>;
#[doc = "Register `DP_LN3_LINK_TRAINING_CTL` writer"]
pub type W = crate::W<DpLn3LinkTrainingCtlSpec>;
#[doc = "Lane 3 output amplitude setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DriveCurrentSet3 {
    #[doc = "3: 1200 mV,"]
    B11 = 3,
    #[doc = "2: 800 mV,"]
    B10 = 2,
    #[doc = "1: 600 mV,"]
    B01 = 1,
    #[doc = "0: 400 mV. This bit's type is R/W."]
    B00 = 0,
}
impl From<DriveCurrentSet3> for u8 {
    #[inline(always)]
    fn from(variant: DriveCurrentSet3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DriveCurrentSet3 {
    type Ux = u8;
}
#[doc = "Field `DRIVE_CURRENT_SET_3` reader - Lane 3 output amplitude setting"]
pub type DriveCurrentSet3R = crate::FieldReader<DriveCurrentSet3>;
impl DriveCurrentSet3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DriveCurrentSet3 {
        match self.bits {
            3 => DriveCurrentSet3::B11,
            2 => DriveCurrentSet3::B10,
            1 => DriveCurrentSet3::B01,
            0 => DriveCurrentSet3::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "1200 mV,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DriveCurrentSet3::B11
    }
    #[doc = "800 mV,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DriveCurrentSet3::B10
    }
    #[doc = "600 mV,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DriveCurrentSet3::B01
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DriveCurrentSet3::B00
    }
}
#[doc = "Field `DRIVE_CURRENT_SET_3` writer - Lane 3 output amplitude setting"]
pub type DriveCurrentSet3W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DriveCurrentSet3>;
impl<'a, REG> DriveCurrentSet3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1200 mV,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet3::B11)
    }
    #[doc = "800 mV,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet3::B10)
    }
    #[doc = "600 mV,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet3::B01)
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet3::B00)
    }
}
#[doc = "Field `MAX_DRIVE_REACH_3` reader - This bit field is set to 1 automatically when \n\nmax driving current level of DP Tx is \n\nreached. For more information, refer to \n\nMAX_PRE_REACH_3. For test purpose only. \n\nThis bit's type is RO."]
pub type MaxDriveReach3R = crate::BitReader;
#[doc = "Lane 3 pre-emphasis level setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PreEmphasisSet3 {
    #[doc = "3: 9.5 dB,"]
    B11 = 3,
    #[doc = "2: 6.0 dB,"]
    B10 = 2,
    #[doc = "1: 3.5 dB,"]
    B01 = 1,
    #[doc = "0: 0 dB (No pre-emphasis). This bit's type is R/W."]
    B00 = 0,
}
impl From<PreEmphasisSet3> for u8 {
    #[inline(always)]
    fn from(variant: PreEmphasisSet3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PreEmphasisSet3 {
    type Ux = u8;
}
#[doc = "Field `PRE_EMPHASIS_SET_3` reader - Lane 3 pre-emphasis level setting"]
pub type PreEmphasisSet3R = crate::FieldReader<PreEmphasisSet3>;
impl PreEmphasisSet3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PreEmphasisSet3 {
        match self.bits {
            3 => PreEmphasisSet3::B11,
            2 => PreEmphasisSet3::B10,
            1 => PreEmphasisSet3::B01,
            0 => PreEmphasisSet3::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "9.5 dB,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == PreEmphasisSet3::B11
    }
    #[doc = "6.0 dB,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == PreEmphasisSet3::B10
    }
    #[doc = "3.5 dB,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == PreEmphasisSet3::B01
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == PreEmphasisSet3::B00
    }
}
#[doc = "Field `PRE_EMPHASIS_SET_3` writer - Lane 3 pre-emphasis level setting"]
pub type PreEmphasisSet3W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PreEmphasisSet3>;
impl<'a, REG> PreEmphasisSet3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "9.5 dB,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet3::B11)
    }
    #[doc = "6.0 dB,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet3::B10)
    }
    #[doc = "3.5 dB,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet3::B01)
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet3::B00)
    }
}
#[doc = "Field `MAX_PRE_REACH_3` reader - This bit field is set to 1 automatically when \n\nmax pre-emphasis level of DP Tx is \n\nreached. \n\nNote that the MAX_PRE_REACH_3 and \n\nMAX_DRIVE_REACH_3 have the same \n\nvalue like the following table. \n\nBoth of MAX_PRE_REACH_3 and \n\nMAX_DRIVE_REACH_3 are for test purpose \n\nonly. \n\nThis bit's type is RO."]
pub type MaxPreReach3R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Lane 3 output amplitude setting"]
    #[inline(always)]
    pub fn drive_current_set_3(&self) -> DriveCurrentSet3R {
        DriveCurrentSet3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This bit field is set to 1 automatically when \n\nmax driving current level of DP Tx is \n\nreached. For more information, refer to \n\nMAX_PRE_REACH_3. For test purpose only. \n\nThis bit's type is RO."]
    #[inline(always)]
    pub fn max_drive_reach_3(&self) -> MaxDriveReach3R {
        MaxDriveReach3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Lane 3 pre-emphasis level setting"]
    #[inline(always)]
    pub fn pre_emphasis_set_3(&self) -> PreEmphasisSet3R {
        PreEmphasisSet3R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - This bit field is set to 1 automatically when \n\nmax pre-emphasis level of DP Tx is \n\nreached. \n\nNote that the MAX_PRE_REACH_3 and \n\nMAX_DRIVE_REACH_3 have the same \n\nvalue like the following table. \n\nBoth of MAX_PRE_REACH_3 and \n\nMAX_DRIVE_REACH_3 are for test purpose \n\nonly. \n\nThis bit's type is RO."]
    #[inline(always)]
    pub fn max_pre_reach_3(&self) -> MaxPreReach3R {
        MaxPreReach3R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lane 3 output amplitude setting"]
    #[inline(always)]
    #[must_use]
    pub fn drive_current_set_3(&mut self) -> DriveCurrentSet3W<DpLn3LinkTrainingCtlSpec> {
        DriveCurrentSet3W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Lane 3 pre-emphasis level setting"]
    #[inline(always)]
    #[must_use]
    pub fn pre_emphasis_set_3(&mut self) -> PreEmphasisSet3W<DpLn3LinkTrainingCtlSpec> {
        PreEmphasisSet3W::new(self, 3)
    }
}
#[doc = "DP Lane 3 Link Training Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_ln3_link_training_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_ln3_link_training_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpLn3LinkTrainingCtlSpec;
impl crate::RegisterSpec for DpLn3LinkTrainingCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_ln3_link_training_ctl::R`](R) reader structure"]
impl crate::Readable for DpLn3LinkTrainingCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_ln3_link_training_ctl::W`](W) writer structure"]
impl crate::Writable for DpLn3LinkTrainingCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets DP_LN3_LINK_TRAINING_CTL to value 0"]
impl crate::Resettable for DpLn3LinkTrainingCtlSpec {
    const RESET_VALUE: u32 = 0;
}
