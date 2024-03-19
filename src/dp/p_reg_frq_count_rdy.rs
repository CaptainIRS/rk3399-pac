#[doc = "Register `P_REG_FRQ_COUNT_RDY` reader"]
pub type R = crate::R<PRegFrqCountRdySpec>;
#[doc = "Register `P_REG_FRQ_COUNT_RDY` writer"]
pub type W = crate::W<PRegFrqCountRdySpec>;
#[doc = "frequency counter ready indicator \n\n(frequency counter for VCO band \n\nselection)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrqCountRdy {
    #[doc = "1: frequency counter ready, its output is the real value of video PLL"]
    B1 = 1,
    #[doc = "0: frequency counter not ready, its output is not the real value"]
    B0 = 0,
}
impl From<FrqCountRdy> for bool {
    #[inline(always)]
    fn from(variant: FrqCountRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRQ_COUNT_RDY` reader - frequency counter ready indicator \n\n(frequency counter for VCO band \n\nselection)"]
pub type FrqCountRdyR = crate::BitReader<FrqCountRdy>;
impl FrqCountRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrqCountRdy {
        match self.bits {
            true => FrqCountRdy::B1,
            false => FrqCountRdy::B0,
        }
    }
    #[doc = "frequency counter ready, its output is the real value of video PLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FrqCountRdy::B1
    }
    #[doc = "frequency counter not ready, its output is not the real value"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FrqCountRdy::B0
    }
}
#[doc = "Field `FRQ_COUNT_RDY` writer - frequency counter ready indicator \n\n(frequency counter for VCO band \n\nselection)"]
pub type FrqCountRdyW<'a, REG> = crate::BitWriter1C<'a, REG, FrqCountRdy>;
impl<'a, REG> FrqCountRdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frequency counter ready, its output is the real value of video PLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FrqCountRdy::B1)
    }
    #[doc = "frequency counter not ready, its output is not the real value"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FrqCountRdy::B0)
    }
}
impl R {
    #[doc = "Bit 0 - frequency counter ready indicator \n\n(frequency counter for VCO band \n\nselection)"]
    #[inline(always)]
    pub fn frq_count_rdy(&self) -> FrqCountRdyR {
        FrqCountRdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - frequency counter ready indicator \n\n(frequency counter for VCO band \n\nselection)"]
    #[inline(always)]
    #[must_use]
    pub fn frq_count_rdy(&mut self) -> FrqCountRdyW<PRegFrqCountRdySpec> {
        FrqCountRdyW::new(self, 0)
    }
}
#[doc = "frequency counter ready indicator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p_reg_frq_count_rdy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p_reg_frq_count_rdy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRegFrqCountRdySpec;
impl crate::RegisterSpec for PRegFrqCountRdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p_reg_frq_count_rdy::R`](R) reader structure"]
impl crate::Readable for PRegFrqCountRdySpec {}
#[doc = "`write(|w| ..)` method takes [`p_reg_frq_count_rdy::W`](W) writer structure"]
impl crate::Writable for PRegFrqCountRdySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets P_REG_FRQ_COUNT_RDY to value 0"]
impl crate::Resettable for PRegFrqCountRdySpec {
    const RESET_VALUE: u32 = 0;
}
