#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "Frame process done interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrmDoneInt {
    #[doc = "0: inactive;"]
    B0 = 0,
    #[doc = "1: active;"]
    B1 = 1,
}
impl From<FrmDoneInt> for bool {
    #[inline(always)]
    fn from(variant: FrmDoneInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRM_DONE_INT` reader - Frame process done interrupt"]
pub type FrmDoneIntR = crate::BitReader<FrmDoneInt>;
impl FrmDoneIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrmDoneInt {
        match self.bits {
            false => FrmDoneInt::B0,
            true => FrmDoneInt::B1,
        }
    }
    #[doc = "inactive;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FrmDoneInt::B0
    }
    #[doc = "active;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FrmDoneInt::B1
    }
}
#[doc = "Frame process done interrupt enable:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrmDoneIntEn {
    #[doc = "0: disable;"]
    B0 = 0,
    #[doc = "1: enable;"]
    B1 = 1,
}
impl From<FrmDoneIntEn> for bool {
    #[inline(always)]
    fn from(variant: FrmDoneIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRM_DONE_INT_EN` reader - Frame process done interrupt enable:"]
pub type FrmDoneIntEnR = crate::BitReader<FrmDoneIntEn>;
impl FrmDoneIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrmDoneIntEn {
        match self.bits {
            false => FrmDoneIntEn::B0,
            true => FrmDoneIntEn::B1,
        }
    }
    #[doc = "disable;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FrmDoneIntEn::B0
    }
    #[doc = "enable;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FrmDoneIntEn::B1
    }
}
#[doc = "Field `FRM_DONE_INT_EN` writer - Frame process done interrupt enable:"]
pub type FrmDoneIntEnW<'a, REG> = crate::BitWriter<'a, REG, FrmDoneIntEn>;
impl<'a, REG> FrmDoneIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FrmDoneIntEn::B0)
    }
    #[doc = "enable;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FrmDoneIntEn::B1)
    }
}
#[doc = "Field `FRM_DONE_INT_CLR` reader - Frame process done interrupt clear\n\nAfter be set to 1, this bit will be clear automatically."]
pub type FrmDoneIntClrR = crate::BitReader;
#[doc = "Field `FRM_DONE_INT_CLR` writer - Frame process done interrupt clear\n\nAfter be set to 1, this bit will be clear automatically."]
pub type FrmDoneIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Frame process done interrupt"]
    #[inline(always)]
    pub fn frm_done_int(&self) -> FrmDoneIntR {
        FrmDoneIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Frame process done interrupt enable:"]
    #[inline(always)]
    pub fn frm_done_int_en(&self) -> FrmDoneIntEnR {
        FrmDoneIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Frame process done interrupt clear\n\nAfter be set to 1, this bit will be clear automatically."]
    #[inline(always)]
    pub fn frm_done_int_clr(&self) -> FrmDoneIntClrR {
        FrmDoneIntClrR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Frame process done interrupt enable:"]
    #[inline(always)]
    #[must_use]
    pub fn frm_done_int_en(&mut self) -> FrmDoneIntEnW<IntSpec> {
        FrmDoneIntEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Frame process done interrupt clear\n\nAfter be set to 1, this bit will be clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn frm_done_int_clr(&mut self) -> FrmDoneIntClrW<IntSpec> {
        FrmDoneIntClrW::new(self, 16)
    }
}
#[doc = "interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0000;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {
    const RESET_VALUE: u32 = 0;
}
