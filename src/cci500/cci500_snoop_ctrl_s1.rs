#[doc = "Register `CCI500_SNOOP_CTRL_S1` reader"]
pub type R = crate::R<Cci500SnoopCtrlS1Spec>;
#[doc = "Register `CCI500_SNOOP_CTRL_S1` writer"]
pub type W = crate::W<Cci500SnoopCtrlS1Spec>;
#[doc = "Enable issuing of snoop requests from this\n\nslave interface.\n\nRAZ/WI for interfaces not supporting snoops:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableSnoops {
    #[doc = "0: Disable snoop requests."]
    B0 = 0,
    #[doc = "1: Enable snoop requests."]
    B1 = 1,
}
impl From<EnableSnoops> for bool {
    #[inline(always)]
    fn from(variant: EnableSnoops) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_SNOOPS` reader - Enable issuing of snoop requests from this\n\nslave interface.\n\nRAZ/WI for interfaces not supporting snoops:"]
pub type EnableSnoopsR = crate::BitReader<EnableSnoops>;
impl EnableSnoopsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableSnoops {
        match self.bits {
            false => EnableSnoops::B0,
            true => EnableSnoops::B1,
        }
    }
    #[doc = "Disable snoop requests."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EnableSnoops::B0
    }
    #[doc = "Enable snoop requests."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EnableSnoops::B1
    }
}
#[doc = "Field `ENABLE_SNOOPS` writer - Enable issuing of snoop requests from this\n\nslave interface.\n\nRAZ/WI for interfaces not supporting snoops:"]
pub type EnableSnoopsW<'a, REG> = crate::BitWriter<'a, REG, EnableSnoops>;
impl<'a, REG> EnableSnoopsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snoop requests."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSnoops::B0)
    }
    #[doc = "Enable snoop requests."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSnoops::B1)
    }
}
#[doc = "Enable issuing of DVM message requests from\n\nthis slave interface.\n\nRAZ/WI for interfaces not supporting DVM\n\nmessages:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableDvms {
    #[doc = "0: Disable DVM message requests."]
    B0 = 0,
    #[doc = "1: Enable DVM message requests."]
    B1 = 1,
}
impl From<EnableDvms> for bool {
    #[inline(always)]
    fn from(variant: EnableDvms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_DVMS` reader - Enable issuing of DVM message requests from\n\nthis slave interface.\n\nRAZ/WI for interfaces not supporting DVM\n\nmessages:"]
pub type EnableDvmsR = crate::BitReader<EnableDvms>;
impl EnableDvmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableDvms {
        match self.bits {
            false => EnableDvms::B0,
            true => EnableDvms::B1,
        }
    }
    #[doc = "Disable DVM message requests."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EnableDvms::B0
    }
    #[doc = "Enable DVM message requests."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EnableDvms::B1
    }
}
#[doc = "Field `ENABLE_DVMS` writer - Enable issuing of DVM message requests from\n\nthis slave interface.\n\nRAZ/WI for interfaces not supporting DVM\n\nmessages:"]
pub type EnableDvmsW<'a, REG> = crate::BitWriter<'a, REG, EnableDvms>;
impl<'a, REG> EnableDvmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DVM message requests."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EnableDvms::B0)
    }
    #[doc = "Enable DVM message requests."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EnableDvms::B1)
    }
}
#[doc = "Field `SUPPORT_SNOOPS` reader - Slave interface supports snoop requests. This\n\nis overridden to 0x0 if you set the\n\nControl Override Register bit\\[0\\]"]
pub type SupportSnoopsR = crate::BitReader;
#[doc = "Field `SUPPORT_SNOOPS` writer - Slave interface supports snoop requests. This\n\nis overridden to 0x0 if you set the\n\nControl Override Register bit\\[0\\]"]
pub type SupportSnoopsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPPORT_DVMS` reader - Slave interface supports DVM messages. This\n\nis overridden to 0x0 if you set the\n\nControl Override Register bit\\[1\\]"]
pub type SupportDvmsR = crate::BitReader;
#[doc = "Field `SUPPORT_DVMS` writer - Slave interface supports DVM messages. This\n\nis overridden to 0x0 if you set the\n\nControl Override Register bit\\[1\\]"]
pub type SupportDvmsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable issuing of snoop requests from this\n\nslave interface.\n\nRAZ/WI for interfaces not supporting snoops:"]
    #[inline(always)]
    pub fn enable_snoops(&self) -> EnableSnoopsR {
        EnableSnoopsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable issuing of DVM message requests from\n\nthis slave interface.\n\nRAZ/WI for interfaces not supporting DVM\n\nmessages:"]
    #[inline(always)]
    pub fn enable_dvms(&self) -> EnableDvmsR {
        EnableDvmsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave interface supports snoop requests. This\n\nis overridden to 0x0 if you set the\n\nControl Override Register bit\\[0\\]"]
    #[inline(always)]
    pub fn support_snoops(&self) -> SupportSnoopsR {
        SupportSnoopsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Slave interface supports DVM messages. This\n\nis overridden to 0x0 if you set the\n\nControl Override Register bit\\[1\\]"]
    #[inline(always)]
    pub fn support_dvms(&self) -> SupportDvmsR {
        SupportDvmsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable issuing of snoop requests from this\n\nslave interface.\n\nRAZ/WI for interfaces not supporting snoops:"]
    #[inline(always)]
    #[must_use]
    pub fn enable_snoops(&mut self) -> EnableSnoopsW<Cci500SnoopCtrlS1Spec> {
        EnableSnoopsW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable issuing of DVM message requests from\n\nthis slave interface.\n\nRAZ/WI for interfaces not supporting DVM\n\nmessages:"]
    #[inline(always)]
    #[must_use]
    pub fn enable_dvms(&mut self) -> EnableDvmsW<Cci500SnoopCtrlS1Spec> {
        EnableDvmsW::new(self, 1)
    }
    #[doc = "Bit 30 - Slave interface supports snoop requests. This\n\nis overridden to 0x0 if you set the\n\nControl Override Register bit\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn support_snoops(&mut self) -> SupportSnoopsW<Cci500SnoopCtrlS1Spec> {
        SupportSnoopsW::new(self, 30)
    }
    #[doc = "Bit 31 - Slave interface supports DVM messages. This\n\nis overridden to 0x0 if you set the\n\nControl Override Register bit\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn support_dvms(&mut self) -> SupportDvmsW<Cci500SnoopCtrlS1Spec> {
        SupportDvmsW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_snoop_ctrl_s1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_snoop_ctrl_s1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500SnoopCtrlS1Spec;
impl crate::RegisterSpec for Cci500SnoopCtrlS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_snoop_ctrl_s1::R`](R) reader structure"]
impl crate::Readable for Cci500SnoopCtrlS1Spec {}
#[doc = "`write(|w| ..)` method takes [`cci500_snoop_ctrl_s1::W`](W) writer structure"]
impl crate::Writable for Cci500SnoopCtrlS1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_SNOOP_CTRL_S1 to value 0"]
impl crate::Resettable for Cci500SnoopCtrlS1Spec {
    const RESET_VALUE: u32 = 0;
}
