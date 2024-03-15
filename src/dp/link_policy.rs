#[doc = "Register `LINK_POLICY` reader"]
pub type R = crate::R<LinkPolicySpec>;
#[doc = "Register `LINK_POLICY` writer"]
pub type W = crate::W<LinkPolicySpec>;
#[doc = "Field `FRAME_CHANGE_EN` reader - Framing change enable"]
pub type FrameChangeEnR = crate::BitReader;
#[doc = "Field `FRAME_CHANGE_EN` writer - Framing change enable"]
pub type FrameChangeEnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LINK_TRAIN_INV` reader - Invert training bit enable"]
pub type LinkTrainInvR = crate::BitReader;
#[doc = "Field `LINK_TRAIN_INV` writer - Invert training bit enable"]
pub type LinkTrainInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINK_TRAIN_405G` reader - 405g training enable"]
pub type LinkTrain405gR = crate::BitReader;
#[doc = "Field `LINK_TRAIN_405G` writer - 405g training enable"]
pub type LinkTrain405gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINK_TRAIN_WR_EN` reader - Training first write en"]
pub type LinkTrainWrEnR = crate::BitReader;
#[doc = "Field `LINK_TRAIN_WR_EN` writer - Training first write en"]
pub type LinkTrainWrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINK_TRAIN_CR_LP_IN` reader - Link training CR loop in"]
pub type LinkTrainCrLpInR = crate::FieldReader;
#[doc = "Field `LINK_TRAIN_CR_LP_IN` writer - Link training CR loop in"]
pub type LinkTrainCrLpInW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ALTERNATE_SR_EN` reader - Alternate SR enable"]
pub type AlternateSrEnR = crate::BitReader;
#[doc = "Field `ALTERNATE_SR_EN` writer - Alternate SR enable"]
pub type AlternateSrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Framing change enable"]
    #[inline(always)]
    pub fn frame_change_en(&self) -> FrameChangeEnR {
        FrameChangeEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invert training bit enable"]
    #[inline(always)]
    pub fn link_train_inv(&self) -> LinkTrainInvR {
        LinkTrainInvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 405g training enable"]
    #[inline(always)]
    pub fn link_train_405g(&self) -> LinkTrain405gR {
        LinkTrain405gR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Training first write en"]
    #[inline(always)]
    pub fn link_train_wr_en(&self) -> LinkTrainWrEnR {
        LinkTrainWrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Link training CR loop in"]
    #[inline(always)]
    pub fn link_train_cr_lp_in(&self) -> LinkTrainCrLpInR {
        LinkTrainCrLpInR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alternate SR enable"]
    #[inline(always)]
    pub fn alternate_sr_en(&self) -> AlternateSrEnR {
        AlternateSrEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Framing change enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame_change_en(&mut self) -> FrameChangeEnW<LinkPolicySpec> {
        FrameChangeEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Invert training bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn link_train_inv(&mut self) -> LinkTrainInvW<LinkPolicySpec> {
        LinkTrainInvW::new(self, 1)
    }
    #[doc = "Bit 2 - 405g training enable"]
    #[inline(always)]
    #[must_use]
    pub fn link_train_405g(&mut self) -> LinkTrain405gW<LinkPolicySpec> {
        LinkTrain405gW::new(self, 2)
    }
    #[doc = "Bit 3 - Training first write en"]
    #[inline(always)]
    #[must_use]
    pub fn link_train_wr_en(&mut self) -> LinkTrainWrEnW<LinkPolicySpec> {
        LinkTrainWrEnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Link training CR loop in"]
    #[inline(always)]
    #[must_use]
    pub fn link_train_cr_lp_in(&mut self) -> LinkTrainCrLpInW<LinkPolicySpec> {
        LinkTrainCrLpInW::new(self, 4)
    }
    #[doc = "Bit 7 - Alternate SR enable"]
    #[inline(always)]
    #[must_use]
    pub fn alternate_sr_en(&mut self) -> AlternateSrEnW<LinkPolicySpec> {
        AlternateSrEnW::new(self, 7)
    }
}
#[doc = "Dp Link Policy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_policy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_policy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkPolicySpec;
impl crate::RegisterSpec for LinkPolicySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_policy::R`](R) reader structure"]
impl crate::Readable for LinkPolicySpec {}
#[doc = "`write(|w| ..)` method takes [`link_policy::W`](W) writer structure"]
impl crate::Writable for LinkPolicySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x71;
}
#[doc = "`reset()` method sets LINK_POLICY to value 0x50"]
impl crate::Resettable for LinkPolicySpec {
    const RESET_VALUE: u32 = 0x50;
}
