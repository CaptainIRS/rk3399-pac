#[doc = "Register `VP_STUFF` reader"]
pub type R = crate::R<VpStuffSpec>;
#[doc = "Register `VP_STUFF` writer"]
pub type W = crate::W<VpStuffSpec>;
#[doc = "Pixel packing stuffing control. The action is stated corresponding to pp_stuffing:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpStuffing {
    #[doc = "0: Pixel packing block in stuffing mode. When \"de_rep\" goes to low the outputs are fixed to 0x00."]
    B0 = 0,
    #[doc = "1: Pixel packing block in stuffing mode. When \"de_rep\" goes to low the outputs are fixed to 0x00."]
    B1 = 1,
}
impl From<PpStuffing> for bool {
    #[inline(always)]
    fn from(variant: PpStuffing) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_STUFFING` reader - Pixel packing stuffing control. The action is stated corresponding to pp_stuffing:"]
pub type PpStuffingR = crate::BitReader<PpStuffing>;
impl PpStuffingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpStuffing {
        match self.bits {
            false => PpStuffing::B0,
            true => PpStuffing::B1,
        }
    }
    #[doc = "Pixel packing block in stuffing mode. When \"de_rep\" goes to low the outputs are fixed to 0x00."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpStuffing::B0
    }
    #[doc = "Pixel packing block in stuffing mode. When \"de_rep\" goes to low the outputs are fixed to 0x00."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpStuffing::B1
    }
}
#[doc = "Field `PP_STUFFING` writer - Pixel packing stuffing control. The action is stated corresponding to pp_stuffing:"]
pub type PpStuffingW<'a, REG> = crate::BitWriter<'a, REG, PpStuffing>;
impl<'a, REG> PpStuffingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pixel packing block in stuffing mode. When \"de_rep\" goes to low the outputs are fixed to 0x00."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PpStuffing::B0)
    }
    #[doc = "Pixel packing block in stuffing mode. When \"de_rep\" goes to low the outputs are fixed to 0x00."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PpStuffing::B1)
    }
}
#[doc = "YCC 422 remap stuffing control. For horizontal blanking, the action is stated corresponding to ycc422_stuffing:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ycc422Stuffing {
    #[doc = "0: YCC 422 remap block in stuffing mode. When \"de\" goes to low the outputs are fixed to 0x00."]
    B0 = 0,
    #[doc = "1: YCC 422 remap block in stuffing mode. When \"de\" goes to low the outputs are fixed to 0x00."]
    B1 = 1,
}
impl From<Ycc422Stuffing> for bool {
    #[inline(always)]
    fn from(variant: Ycc422Stuffing) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `YCC422_STUFFING` reader - YCC 422 remap stuffing control. For horizontal blanking, the action is stated corresponding to ycc422_stuffing:"]
pub type Ycc422StuffingR = crate::BitReader<Ycc422Stuffing>;
impl Ycc422StuffingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ycc422Stuffing {
        match self.bits {
            false => Ycc422Stuffing::B0,
            true => Ycc422Stuffing::B1,
        }
    }
    #[doc = "YCC 422 remap block in stuffing mode. When \"de\" goes to low the outputs are fixed to 0x00."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ycc422Stuffing::B0
    }
    #[doc = "YCC 422 remap block in stuffing mode. When \"de\" goes to low the outputs are fixed to 0x00."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ycc422Stuffing::B1
    }
}
#[doc = "Field `YCC422_STUFFING` writer - YCC 422 remap stuffing control. For horizontal blanking, the action is stated corresponding to ycc422_stuffing:"]
pub type Ycc422StuffingW<'a, REG> = crate::BitWriter<'a, REG, Ycc422Stuffing>;
impl<'a, REG> Ycc422StuffingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "YCC 422 remap block in stuffing mode. When \"de\" goes to low the outputs are fixed to 0x00."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ycc422Stuffing::B0)
    }
    #[doc = "YCC 422 remap block in stuffing mode. When \"de\" goes to low the outputs are fixed to 0x00."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ycc422Stuffing::B1)
    }
}
#[doc = "Field `ICX_GOTO_P0_ST` reader - Reserved. Controls packing machine strategy"]
pub type IcxGotoP0StR = crate::BitReader;
#[doc = "Field `ICX_GOTO_P0_ST` writer - Reserved. Controls packing machine strategy"]
pub type IcxGotoP0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFIX_PP_TO_LAST` reader - Reserved. Controls packing machine strategy"]
pub type IfixPpToLastR = crate::BitReader;
#[doc = "Field `IFIX_PP_TO_LAST` writer - Reserved. Controls packing machine strategy"]
pub type IfixPpToLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDEFAULT_PHASE` reader - Controls the default phase packing machine used according to HDMI 1.4b specification: \"If the transmitted video format has timing such that the phase of the first pixel of every Video Data Period corresponds to pixel packing phase 0 (e.g. 10P0, 12P0, 16P0), the Source may set the Default_Phase bit in the GCP. The Sink may use this bit to optimize its filtering or handling of the PP field.\" This means that for 10-bit mode the Htotal must be dividable by 4; for 12- bit mode, the Htotal must be divisible by 2."]
pub type IdefaultPhaseR = crate::BitReader;
#[doc = "Field `IDEFAULT_PHASE` writer - Controls the default phase packing machine used according to HDMI 1.4b specification: \"If the transmitted video format has timing such that the phase of the first pixel of every Video Data Period corresponds to pixel packing phase 0 (e.g. 10P0, 12P0, 16P0), the Source may set the Default_Phase bit in the GCP. The Sink may use this bit to optimize its filtering or handling of the PP field.\" This means that for 10-bit mode the Htotal must be dividable by 4; for 12- bit mode, the Htotal must be divisible by 2."]
pub type IdefaultPhaseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Pixel packing stuffing control. The action is stated corresponding to pp_stuffing:"]
    #[inline(always)]
    pub fn pp_stuffing(&self) -> PpStuffingR {
        PpStuffingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - YCC 422 remap stuffing control. For horizontal blanking, the action is stated corresponding to ycc422_stuffing:"]
    #[inline(always)]
    pub fn ycc422_stuffing(&self) -> Ycc422StuffingR {
        Ycc422StuffingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved. Controls packing machine strategy"]
    #[inline(always)]
    pub fn icx_goto_p0_st(&self) -> IcxGotoP0StR {
        IcxGotoP0StR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved. Controls packing machine strategy"]
    #[inline(always)]
    pub fn ifix_pp_to_last(&self) -> IfixPpToLastR {
        IfixPpToLastR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Controls the default phase packing machine used according to HDMI 1.4b specification: \"If the transmitted video format has timing such that the phase of the first pixel of every Video Data Period corresponds to pixel packing phase 0 (e.g. 10P0, 12P0, 16P0), the Source may set the Default_Phase bit in the GCP. The Sink may use this bit to optimize its filtering or handling of the PP field.\" This means that for 10-bit mode the Htotal must be dividable by 4; for 12- bit mode, the Htotal must be divisible by 2."]
    #[inline(always)]
    pub fn idefault_phase(&self) -> IdefaultPhaseR {
        IdefaultPhaseR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Pixel packing stuffing control. The action is stated corresponding to pp_stuffing:"]
    #[inline(always)]
    #[must_use]
    pub fn pp_stuffing(&mut self) -> PpStuffingW<VpStuffSpec> {
        PpStuffingW::new(self, 1)
    }
    #[doc = "Bit 2 - YCC 422 remap stuffing control. For horizontal blanking, the action is stated corresponding to ycc422_stuffing:"]
    #[inline(always)]
    #[must_use]
    pub fn ycc422_stuffing(&mut self) -> Ycc422StuffingW<VpStuffSpec> {
        Ycc422StuffingW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved. Controls packing machine strategy"]
    #[inline(always)]
    #[must_use]
    pub fn icx_goto_p0_st(&mut self) -> IcxGotoP0StW<VpStuffSpec> {
        IcxGotoP0StW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved. Controls packing machine strategy"]
    #[inline(always)]
    #[must_use]
    pub fn ifix_pp_to_last(&mut self) -> IfixPpToLastW<VpStuffSpec> {
        IfixPpToLastW::new(self, 4)
    }
    #[doc = "Bit 5 - Controls the default phase packing machine used according to HDMI 1.4b specification: \"If the transmitted video format has timing such that the phase of the first pixel of every Video Data Period corresponds to pixel packing phase 0 (e.g. 10P0, 12P0, 16P0), the Source may set the Default_Phase bit in the GCP. The Sink may use this bit to optimize its filtering or handling of the PP field.\" This means that for 10-bit mode the Htotal must be dividable by 4; for 12- bit mode, the Htotal must be divisible by 2."]
    #[inline(always)]
    #[must_use]
    pub fn idefault_phase(&mut self) -> IdefaultPhaseW<VpStuffSpec> {
        IdefaultPhaseW::new(self, 5)
    }
}
#[doc = "Pixel packing stuffing control. The action is stated corresponding to pp_stuffing: 0b: Pixel packing block in direct mode (input blanking data goes directly to output). 1b: Pixel packing block in stuffing mode. When \"de_rep\" goes to low the outputs are fixed to 0x00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_stuff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_stuff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VpStuffSpec;
impl crate::RegisterSpec for VpStuffSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vp_stuff::R`](R) reader structure"]
impl crate::Readable for VpStuffSpec {}
#[doc = "`write(|w| ..)` method takes [`vp_stuff::W`](W) writer structure"]
impl crate::Writable for VpStuffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets VP_STUFF to value 0"]
impl crate::Resettable for VpStuffSpec {
    const RESET_VALUE: u8 = 0;
}
