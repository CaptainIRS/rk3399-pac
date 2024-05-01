#[doc = "Register `SWREG40` reader"]
pub type R = crate::R<Swreg40Spec>;
#[doc = "Register `SWREG40` writer"]
pub type W = crate::W<Swreg40Spec>;
#[doc = "Field `SW_PP_IRQ` reader - the pp finish interrupt request\n\nafter sw query this interrupt,shoud write 0 to reset.\n\nthis bit will no used in pipeline mode"]
pub type SwPpIrqR = crate::BitReader;
#[doc = "Field `SW_PP_IRQ` writer - the pp finish interrupt request\n\nafter sw query this interrupt,shoud write 0 to reset.\n\nthis bit will no used in pipeline mode"]
pub type SwPpIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "the pp finish interrupt request diable flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpIrqDis {
    #[doc = "1: use polling to see the interrupt"]
    B1 = 1,
    #[doc = "0: use sw_pp_irq"]
    B0 = 0,
}
impl From<SwPpIrqDis> for bool {
    #[inline(always)]
    fn from(variant: SwPpIrqDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_IRQ_DIS` reader - the pp finish interrupt request diable flag"]
pub type SwPpIrqDisR = crate::BitReader<SwPpIrqDis>;
impl SwPpIrqDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpIrqDis {
        match self.bits {
            true => SwPpIrqDis::B1,
            false => SwPpIrqDis::B0,
        }
    }
    #[doc = "use polling to see the interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpIrqDis::B1
    }
    #[doc = "use sw_pp_irq"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpIrqDis::B0
    }
}
#[doc = "Field `SW_PP_IRQ_DIS` writer - the pp finish interrupt request diable flag"]
pub type SwPpIrqDisW<'a, REG> = crate::BitWriter<'a, REG, SwPpIrqDis>;
impl<'a, REG> SwPpIrqDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use polling to see the interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpIrqDis::B1)
    }
    #[doc = "use sw_pp_irq"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpIrqDis::B0)
    }
}
#[doc = "Field `SW_PP_RDY_STS` reader - the Interrupt status bit for tell sw processed a picture"]
pub type SwPpRdyStsR = crate::BitReader;
#[doc = "Field `SW_PP_RDY_STS` writer - the Interrupt status bit for tell sw processed a picture"]
pub type SwPpRdyStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_PP_BUS_STS` reader - the Interrupt status bit for tell sw bus have some error"]
pub type SwPpBusStsR = crate::BitReader;
#[doc = "Field `SW_PP_BUS_STS` writer - the Interrupt status bit for tell sw bus have some error"]
pub type SwPpBusStsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the pp finish interrupt request\n\nafter sw query this interrupt,shoud write 0 to reset.\n\nthis bit will no used in pipeline mode"]
    #[inline(always)]
    pub fn sw_pp_irq(&self) -> SwPpIrqR {
        SwPpIrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the pp finish interrupt request diable flag"]
    #[inline(always)]
    pub fn sw_pp_irq_dis(&self) -> SwPpIrqDisR {
        SwPpIrqDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the Interrupt status bit for tell sw processed a picture"]
    #[inline(always)]
    pub fn sw_pp_rdy_sts(&self) -> SwPpRdyStsR {
        SwPpRdyStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the Interrupt status bit for tell sw bus have some error"]
    #[inline(always)]
    pub fn sw_pp_bus_sts(&self) -> SwPpBusStsR {
        SwPpBusStsR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the pp finish interrupt request\n\nafter sw query this interrupt,shoud write 0 to reset.\n\nthis bit will no used in pipeline mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_irq(&mut self) -> SwPpIrqW<Swreg40Spec> {
        SwPpIrqW::new(self, 0)
    }
    #[doc = "Bit 1 - the pp finish interrupt request diable flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_irq_dis(&mut self) -> SwPpIrqDisW<Swreg40Spec> {
        SwPpIrqDisW::new(self, 1)
    }
    #[doc = "Bit 2 - the Interrupt status bit for tell sw processed a picture"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_rdy_sts(&mut self) -> SwPpRdyStsW<Swreg40Spec> {
        SwPpRdyStsW::new(self, 2)
    }
    #[doc = "Bit 3 - the Interrupt status bit for tell sw bus have some error"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_bus_sts(&mut self) -> SwPpBusStsW<Swreg40Spec> {
        SwPpBusStsW::new(self, 3)
    }
}
#[doc = "pp int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg40Spec;
impl crate::RegisterSpec for Swreg40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg40::R`](R) reader structure"]
impl crate::Readable for Swreg40Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg40::W`](W) writer structure"]
impl crate::Writable for Swreg40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG40 to value 0"]
impl crate::Resettable for Swreg40Spec {
    const RESET_VALUE: u32 = 0;
}
