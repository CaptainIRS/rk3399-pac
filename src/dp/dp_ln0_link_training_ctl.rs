#[doc = "Register `DP_LN0_LINK_TRAINING_CTL` reader"]
pub type R = crate::R<DpLn0LinkTrainingCtlSpec>;
#[doc = "Register `DP_LN0_LINK_TRAINING_CTL` writer"]
pub type W = crate::W<DpLn0LinkTrainingCtlSpec>;
#[doc = "Lane 0 output amplitude setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DriveCurrentSet0 {
    #[doc = "3: 1200 mV,"]
    B11 = 3,
    #[doc = "2: 800 mV,"]
    B10 = 2,
    #[doc = "1: 600 mV,"]
    B01 = 1,
    #[doc = "0: 400 mV. This bit's type is R/W."]
    B00 = 0,
}
impl From<DriveCurrentSet0> for u8 {
    #[inline(always)]
    fn from(variant: DriveCurrentSet0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DriveCurrentSet0 {
    type Ux = u8;
}
#[doc = "Field `DRIVE_CURRENT_SET_0` reader - Lane 0 output amplitude setting"]
pub type DriveCurrentSet0R = crate::FieldReader<DriveCurrentSet0>;
impl DriveCurrentSet0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DriveCurrentSet0 {
        match self.bits {
            3 => DriveCurrentSet0::B11,
            2 => DriveCurrentSet0::B10,
            1 => DriveCurrentSet0::B01,
            0 => DriveCurrentSet0::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "1200 mV,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DriveCurrentSet0::B11
    }
    #[doc = "800 mV,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DriveCurrentSet0::B10
    }
    #[doc = "600 mV,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DriveCurrentSet0::B01
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DriveCurrentSet0::B00
    }
}
#[doc = "Field `DRIVE_CURRENT_SET_0` writer - Lane 0 output amplitude setting"]
pub type DriveCurrentSet0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DriveCurrentSet0>;
impl<'a, REG> DriveCurrentSet0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1200 mV,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet0::B11)
    }
    #[doc = "800 mV,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet0::B10)
    }
    #[doc = "600 mV,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet0::B01)
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet0::B00)
    }
}
#[doc = "Field `MAX_DRIVE_REACH_0` reader - This bit field is set to 1 automatically when \n\nmax driving current level of DP Tx is \n\nreached. For test purpose only. This bit's \n\ntype is RO. For more information, refer to \n\nMAX_PRE_REACH_0."]
pub type MaxDriveReach0R = crate::BitReader;
#[doc = "Lane 0 pre-emphasis level setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PreEmphasisSet0 {
    #[doc = "3: 9.5 dB,"]
    B11 = 3,
    #[doc = "2: 6.0 dB,"]
    B10 = 2,
    #[doc = "1: 3.5 dB,"]
    B01 = 1,
    #[doc = "0: 0 dB (No pre-emphasis). This bit's type is R/W."]
    B00 = 0,
}
impl From<PreEmphasisSet0> for u8 {
    #[inline(always)]
    fn from(variant: PreEmphasisSet0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PreEmphasisSet0 {
    type Ux = u8;
}
#[doc = "Field `PRE_EMPHASIS_SET_0` reader - Lane 0 pre-emphasis level setting"]
pub type PreEmphasisSet0R = crate::FieldReader<PreEmphasisSet0>;
impl PreEmphasisSet0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PreEmphasisSet0 {
        match self.bits {
            3 => PreEmphasisSet0::B11,
            2 => PreEmphasisSet0::B10,
            1 => PreEmphasisSet0::B01,
            0 => PreEmphasisSet0::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "9.5 dB,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == PreEmphasisSet0::B11
    }
    #[doc = "6.0 dB,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == PreEmphasisSet0::B10
    }
    #[doc = "3.5 dB,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == PreEmphasisSet0::B01
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == PreEmphasisSet0::B00
    }
}
#[doc = "Field `PRE_EMPHASIS_SET_0` writer - Lane 0 pre-emphasis level setting"]
pub type PreEmphasisSet0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PreEmphasisSet0>;
impl<'a, REG> PreEmphasisSet0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "9.5 dB,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet0::B11)
    }
    #[doc = "6.0 dB,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet0::B10)
    }
    #[doc = "3.5 dB,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet0::B01)
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet0::B00)
    }
}
#[doc = "Field `MAX_PRE_REACH_0` reader - This bit field is set to 1 automatically when \n\nmax pre-emphasis level of DP Tx is \n\nreached. \n\nThis bit's type is RO. \n\nNote that the MAX_PRE_REACH_0 and \n\nMAX_DRIVE_REACH_0 have the same \n\nvalue like the following table."]
pub type MaxPreReach0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Lane 0 output amplitude setting"]
    #[inline(always)]
    pub fn drive_current_set_0(&self) -> DriveCurrentSet0R {
        DriveCurrentSet0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This bit field is set to 1 automatically when \n\nmax driving current level of DP Tx is \n\nreached. For test purpose only. This bit's \n\ntype is RO. For more information, refer to \n\nMAX_PRE_REACH_0."]
    #[inline(always)]
    pub fn max_drive_reach_0(&self) -> MaxDriveReach0R {
        MaxDriveReach0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Lane 0 pre-emphasis level setting"]
    #[inline(always)]
    pub fn pre_emphasis_set_0(&self) -> PreEmphasisSet0R {
        PreEmphasisSet0R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - This bit field is set to 1 automatically when \n\nmax pre-emphasis level of DP Tx is \n\nreached. \n\nThis bit's type is RO. \n\nNote that the MAX_PRE_REACH_0 and \n\nMAX_DRIVE_REACH_0 have the same \n\nvalue like the following table."]
    #[inline(always)]
    pub fn max_pre_reach_0(&self) -> MaxPreReach0R {
        MaxPreReach0R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lane 0 output amplitude setting"]
    #[inline(always)]
    #[must_use]
    pub fn drive_current_set_0(&mut self) -> DriveCurrentSet0W<DpLn0LinkTrainingCtlSpec> {
        DriveCurrentSet0W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Lane 0 pre-emphasis level setting"]
    #[inline(always)]
    #[must_use]
    pub fn pre_emphasis_set_0(&mut self) -> PreEmphasisSet0W<DpLn0LinkTrainingCtlSpec> {
        PreEmphasisSet0W::new(self, 3)
    }
}
#[doc = "DP Lane 0 Link Training Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_ln0_link_training_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_ln0_link_training_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpLn0LinkTrainingCtlSpec;
impl crate::RegisterSpec for DpLn0LinkTrainingCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_ln0_link_training_ctl::R`](R) reader structure"]
impl crate::Readable for DpLn0LinkTrainingCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_ln0_link_training_ctl::W`](W) writer structure"]
impl crate::Writable for DpLn0LinkTrainingCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets DP_LN0_LINK_TRAINING_CTL to value 0"]
impl crate::Resettable for DpLn0LinkTrainingCtlSpec {
    const RESET_VALUE: u32 = 0;
}
