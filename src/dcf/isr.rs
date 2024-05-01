#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "DCF done\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcfDone {
    #[doc = "1: DDR change frequency completed;"]
    B1 = 1,
    #[doc = "0: DDR change frequency not completed;"]
    B0 = 0,
}
impl From<DcfDone> for bool {
    #[inline(always)]
    fn from(variant: DcfDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCF_DONE` reader - DCF done"]
pub type DcfDoneR = crate::BitReader<DcfDone>;
impl DcfDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcfDone {
        match self.bits {
            true => DcfDone::B1,
            false => DcfDone::B0,
        }
    }
    #[doc = "DDR change frequency completed;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DcfDone::B1
    }
    #[doc = "DDR change frequency not completed;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DcfDone::B0
    }
}
#[doc = "Field `DCF_DONE` writer - DCF done"]
pub type DcfDoneW<'a, REG> = crate::BitWriter1C<'a, REG, DcfDone>;
impl<'a, REG> DcfDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DDR change frequency completed;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DcfDone::B1)
    }
    #[doc = "DDR change frequency not completed;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DcfDone::B0)
    }
}
#[doc = "DCF error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcfError {
    #[doc = "1: error response during DDR change frequency ;"]
    B1 = 1,
    #[doc = "0: no error response during DDR change frequency ;"]
    B0 = 0,
}
impl From<DcfError> for bool {
    #[inline(always)]
    fn from(variant: DcfError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCF_ERROR` reader - DCF error"]
pub type DcfErrorR = crate::BitReader<DcfError>;
impl DcfErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcfError {
        match self.bits {
            true => DcfError::B1,
            false => DcfError::B0,
        }
    }
    #[doc = "error response during DDR change frequency ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DcfError::B1
    }
    #[doc = "no error response during DDR change frequency ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DcfError::B0
    }
}
#[doc = "Field `DCF_ERROR` writer - DCF error"]
pub type DcfErrorW<'a, REG> = crate::BitWriter1C<'a, REG, DcfError>;
impl<'a, REG> DcfErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response during DDR change frequency ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DcfError::B1)
    }
    #[doc = "no error response during DDR change frequency ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DcfError::B0)
    }
}
#[doc = "Field `TIMEOUT` reader - timeout\n\ntimeout status\n\n1'b1 : An timeout error occurred according to the toset value"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - timeout\n\ntimeout status\n\n1'b1 : An timeout error occurred according to the toset value"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCF done"]
    #[inline(always)]
    pub fn dcf_done(&self) -> DcfDoneR {
        DcfDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCF error"]
    #[inline(always)]
    pub fn dcf_error(&self) -> DcfErrorR {
        DcfErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - timeout\n\ntimeout status\n\n1'b1 : An timeout error occurred according to the toset value"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCF done"]
    #[inline(always)]
    #[must_use]
    pub fn dcf_done(&mut self) -> DcfDoneW<IsrSpec> {
        DcfDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - DCF error"]
    #[inline(always)]
    #[must_use]
    pub fn dcf_error(&mut self) -> DcfErrorW<IsrSpec> {
        DcfErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - timeout\n\ntimeout status\n\n1'b1 : An timeout error occurred according to the toset value"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<IsrSpec> {
        TimeoutW::new(self, 2)
    }
}
#[doc = "DCF Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
