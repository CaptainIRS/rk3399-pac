#[doc = "Register `ANALOG_CTL_2` reader"]
pub type R = crate::R<AnalogCtl2Spec>;
#[doc = "Register `ANALOG_CTL_2` writer"]
pub type W = crate::W<AnalogCtl2Spec>;
#[doc = "Choose the reference clock of PHY use \n\n24M or 27M:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel24m {
    #[doc = "1: Use 24M clock,"]
    B1 = 1,
    #[doc = "0: Use 27M clock."]
    B0 = 0,
}
impl From<Sel24m> for bool {
    #[inline(always)]
    fn from(variant: Sel24m) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL_24M` reader - Choose the reference clock of PHY use \n\n24M or 27M:"]
pub type Sel24mR = crate::BitReader<Sel24m>;
impl Sel24mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel24m {
        match self.bits {
            true => Sel24m::B1,
            false => Sel24m::B0,
        }
    }
    #[doc = "Use 24M clock,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sel24m::B1
    }
    #[doc = "Use 27M clock."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sel24m::B0
    }
}
#[doc = "Field `SEL_24M` writer - Choose the reference clock of PHY use \n\n24M or 27M:"]
pub type Sel24mW<'a, REG> = crate::BitWriter<'a, REG, Sel24m>;
impl<'a, REG> Sel24mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 24M clock,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel24m::B1)
    }
    #[doc = "Use 27M clock."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel24m::B0)
    }
}
impl R {
    #[doc = "Bit 3 - Choose the reference clock of PHY use \n\n24M or 27M:"]
    #[inline(always)]
    pub fn sel_24m(&self) -> Sel24mR {
        Sel24mR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Choose the reference clock of PHY use \n\n24M or 27M:"]
    #[inline(always)]
    #[must_use]
    pub fn sel_24m(&mut self) -> Sel24mW<AnalogCtl2Spec> {
        Sel24mW::new(self, 3)
    }
}
#[doc = "Analog Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl2Spec;
impl crate::RegisterSpec for AnalogCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_2::R`](R) reader structure"]
impl crate::Readable for AnalogCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_2::W`](W) writer structure"]
impl crate::Writable for AnalogCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CTL_2 to value 0x08"]
impl crate::Resettable for AnalogCtl2Spec {
    const RESET_VALUE: u32 = 0x08;
}
