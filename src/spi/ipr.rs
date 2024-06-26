#[doc = "Register `IPR` reader"]
pub type R = crate::R<IprSpec>;
#[doc = "Register `IPR` writer"]
pub type W = crate::W<IprSpec>;
#[doc = "Interrupt Polarity\n\nInterrupt Polarity Register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipr {
    #[doc = "0: Active Interrupt Polarity Level is HIGH"]
    B0 = 0,
    #[doc = "1: Active Interrupt Polarity Level is LOW"]
    B1 = 1,
}
impl From<Ipr> for bool {
    #[inline(always)]
    fn from(variant: Ipr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPR` reader - Interrupt Polarity\n\nInterrupt Polarity Register"]
pub type IprR = crate::BitReader<Ipr>;
impl IprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipr {
        match self.bits {
            false => Ipr::B0,
            true => Ipr::B1,
        }
    }
    #[doc = "Active Interrupt Polarity Level is HIGH"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ipr::B0
    }
    #[doc = "Active Interrupt Polarity Level is LOW"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ipr::B1
    }
}
#[doc = "Field `IPR` writer - Interrupt Polarity\n\nInterrupt Polarity Register"]
pub type IprW<'a, REG> = crate::BitWriter<'a, REG, Ipr>;
impl<'a, REG> IprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active Interrupt Polarity Level is HIGH"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ipr::B0)
    }
    #[doc = "Active Interrupt Polarity Level is LOW"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ipr::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Polarity\n\nInterrupt Polarity Register"]
    #[inline(always)]
    pub fn ipr(&self) -> IprR {
        IprR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Polarity\n\nInterrupt Polarity Register"]
    #[inline(always)]
    #[must_use]
    pub fn ipr(&mut self) -> IprW<IprSpec> {
        IprW::new(self, 0)
    }
}
#[doc = "Interrupt Polarity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IprSpec;
impl crate::RegisterSpec for IprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipr::R`](R) reader structure"]
impl crate::Readable for IprSpec {}
#[doc = "`write(|w| ..)` method takes [`ipr::W`](W) writer structure"]
impl crate::Writable for IprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPR to value 0"]
impl crate::Resettable for IprSpec {
    const RESET_VALUE: u32 = 0;
}
