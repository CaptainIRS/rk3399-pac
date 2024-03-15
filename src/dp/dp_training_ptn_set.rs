#[doc = "Register `DP_TRAINING_PTN_SET` reader"]
pub type R = crate::R<DpTrainingPtnSetSpec>;
#[doc = "Register `DP_TRAINING_PTN_SET` writer"]
pub type W = crate::W<DpTrainingPtnSetSpec>;
#[doc = "Link training pattern setting. SW_TRAINING_PATTERN_SET has higher priority than LINK_QUAL_PATTER_SET.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwTrainingPatternSet {
    #[doc = "3: Training pattern not sent"]
    B11 = 3,
    #[doc = "2: Training pattern not sent"]
    B10 = 2,
    #[doc = "1: Training pattern not sent"]
    B01 = 1,
    #[doc = "0: Training pattern not sent"]
    B00 = 0,
}
impl From<SwTrainingPatternSet> for u8 {
    #[inline(always)]
    fn from(variant: SwTrainingPatternSet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwTrainingPatternSet {
    type Ux = u8;
}
#[doc = "Field `SW_TRAINING_PATTERN_SET` reader - Link training pattern setting. SW_TRAINING_PATTERN_SET has higher priority than LINK_QUAL_PATTER_SET."]
pub type SwTrainingPatternSetR = crate::FieldReader<SwTrainingPatternSet>;
impl SwTrainingPatternSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwTrainingPatternSet {
        match self.bits {
            3 => SwTrainingPatternSet::B11,
            2 => SwTrainingPatternSet::B10,
            1 => SwTrainingPatternSet::B01,
            0 => SwTrainingPatternSet::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "Training pattern not sent"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SwTrainingPatternSet::B11
    }
    #[doc = "Training pattern not sent"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwTrainingPatternSet::B10
    }
    #[doc = "Training pattern not sent"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwTrainingPatternSet::B01
    }
    #[doc = "Training pattern not sent"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwTrainingPatternSet::B00
    }
}
#[doc = "Field `SW_TRAINING_PATTERN_SET` writer - Link training pattern setting. SW_TRAINING_PATTERN_SET has higher priority than LINK_QUAL_PATTER_SET."]
pub type SwTrainingPatternSetW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwTrainingPatternSet>;
impl<'a, REG> SwTrainingPatternSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Training pattern not sent"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SwTrainingPatternSet::B11)
    }
    #[doc = "Training pattern not sent"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwTrainingPatternSet::B10)
    }
    #[doc = "Training pattern not sent"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwTrainingPatternSet::B01)
    }
    #[doc = "Training pattern not sent"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwTrainingPatternSet::B00)
    }
}
#[doc = "Field `LINK_QUAL_PATTERN_SET` reader - Link quality pattern setting. 101 = HBR2 Compliance 100 = 80 bit test pattern 011 = PRBS 7 bit 010 = symbol error rate measurement pattern is sent; 001 = D10.2 test pattern is sent; 000= link quality test pattern not sent"]
pub type LinkQualPatternSetR = crate::FieldReader;
#[doc = "Field `LINK_QUAL_PATTERN_SET` writer - Link quality pattern setting. 101 = HBR2 Compliance 100 = 80 bit test pattern 011 = PRBS 7 bit 010 = symbol error rate measurement pattern is sent; 001 = D10.2 test pattern is sent; 000= link quality test pattern not sent"]
pub type LinkQualPatternSetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Disable scramble\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScramblingDisable {
    #[doc = "1: Normal operation"]
    B1 = 1,
    #[doc = "0: Normal operation"]
    B0 = 0,
}
impl From<ScramblingDisable> for bool {
    #[inline(always)]
    fn from(variant: ScramblingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCRAMBLING_DISABLE` reader - Disable scramble"]
pub type ScramblingDisableR = crate::BitReader<ScramblingDisable>;
impl ScramblingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScramblingDisable {
        match self.bits {
            true => ScramblingDisable::B1,
            false => ScramblingDisable::B0,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScramblingDisable::B1
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScramblingDisable::B0
    }
}
#[doc = "Field `SCRAMBLING_DISABLE` writer - Disable scramble"]
pub type ScramblingDisableW<'a, REG> = crate::BitWriter1C<'a, REG, ScramblingDisable>;
impl<'a, REG> ScramblingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScramblingDisable::B1)
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScramblingDisable::B0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Link training pattern setting. SW_TRAINING_PATTERN_SET has higher priority than LINK_QUAL_PATTER_SET."]
    #[inline(always)]
    pub fn sw_training_pattern_set(&self) -> SwTrainingPatternSetR {
        SwTrainingPatternSetR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Link quality pattern setting. 101 = HBR2 Compliance 100 = 80 bit test pattern 011 = PRBS 7 bit 010 = symbol error rate measurement pattern is sent; 001 = D10.2 test pattern is sent; 000= link quality test pattern not sent"]
    #[inline(always)]
    pub fn link_qual_pattern_set(&self) -> LinkQualPatternSetR {
        LinkQualPatternSetR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Disable scramble"]
    #[inline(always)]
    pub fn scrambling_disable(&self) -> ScramblingDisableR {
        ScramblingDisableR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Link training pattern setting. SW_TRAINING_PATTERN_SET has higher priority than LINK_QUAL_PATTER_SET."]
    #[inline(always)]
    #[must_use]
    pub fn sw_training_pattern_set(&mut self) -> SwTrainingPatternSetW<DpTrainingPtnSetSpec> {
        SwTrainingPatternSetW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Link quality pattern setting. 101 = HBR2 Compliance 100 = 80 bit test pattern 011 = PRBS 7 bit 010 = symbol error rate measurement pattern is sent; 001 = D10.2 test pattern is sent; 000= link quality test pattern not sent"]
    #[inline(always)]
    #[must_use]
    pub fn link_qual_pattern_set(&mut self) -> LinkQualPatternSetW<DpTrainingPtnSetSpec> {
        LinkQualPatternSetW::new(self, 2)
    }
    #[doc = "Bit 5 - Disable scramble"]
    #[inline(always)]
    #[must_use]
    pub fn scrambling_disable(&mut self) -> ScramblingDisableW<DpTrainingPtnSetSpec> {
        ScramblingDisableW::new(self, 5)
    }
}
#[doc = "DP Training Pattern Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_training_ptn_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_training_ptn_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpTrainingPtnSetSpec;
impl crate::RegisterSpec for DpTrainingPtnSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_training_ptn_set::R`](R) reader structure"]
impl crate::Readable for DpTrainingPtnSetSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_training_ptn_set::W`](W) writer structure"]
impl crate::Writable for DpTrainingPtnSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x20;
}
#[doc = "`reset()` method sets DP_TRAINING_PTN_SET to value 0"]
impl crate::Resettable for DpTrainingPtnSetSpec {
    const RESET_VALUE: u32 = 0;
}
