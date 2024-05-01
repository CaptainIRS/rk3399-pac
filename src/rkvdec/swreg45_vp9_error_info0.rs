#[doc = "Register `SWREG45_VP9_ERROR_INFO0` reader"]
pub type R = crate::R<Swreg45Vp9ErrorInfo0Spec>;
#[doc = "Register `SWREG45_VP9_ERROR_INFO0` writer"]
pub type W = crate::W<Swreg45Vp9ErrorInfo0Spec>;
#[doc = "vp9_error_info0\n\nfor vp9 16 tile cols, every cols contains 4bits\n\nvp9_error_info0\\[3:0\\]
is for col 0\n\nvp9_error_info0\\[1:0\\]
is to tell tile_rows_cnt\\[1:0\\]\n\nvp9_error_info0\\[3:2\\]
is to tell the error type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Vp9ErrorInfo0 {
    #[doc = "0: no error"]
    B00 = 0,
    #[doc = "1: tilesize error"]
    B01 = 1,
    #[doc = "2: seg skip error"]
    B10 = 2,
    #[doc = "3: ref scale error"]
    B11 = 3,
}
impl From<Vp9ErrorInfo0> for u32 {
    #[inline(always)]
    fn from(variant: Vp9ErrorInfo0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vp9ErrorInfo0 {
    type Ux = u32;
}
#[doc = "Field `VP9_ERROR_INFO0` reader - vp9_error_info0\n\nfor vp9 16 tile cols, every cols contains 4bits\n\nvp9_error_info0\\[3:0\\]
is for col 0\n\nvp9_error_info0\\[1:0\\]
is to tell tile_rows_cnt\\[1:0\\]\n\nvp9_error_info0\\[3:2\\]
is to tell the error type"]
pub type Vp9ErrorInfo0R = crate::FieldReader<Vp9ErrorInfo0>;
impl Vp9ErrorInfo0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vp9ErrorInfo0> {
        match self.bits {
            0 => Some(Vp9ErrorInfo0::B00),
            1 => Some(Vp9ErrorInfo0::B01),
            2 => Some(Vp9ErrorInfo0::B10),
            3 => Some(Vp9ErrorInfo0::B11),
            _ => None,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Vp9ErrorInfo0::B00
    }
    #[doc = "tilesize error"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Vp9ErrorInfo0::B01
    }
    #[doc = "seg skip error"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Vp9ErrorInfo0::B10
    }
    #[doc = "ref scale error"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Vp9ErrorInfo0::B11
    }
}
#[doc = "Field `VP9_ERROR_INFO0` writer - vp9_error_info0\n\nfor vp9 16 tile cols, every cols contains 4bits\n\nvp9_error_info0\\[3:0\\]
is for col 0\n\nvp9_error_info0\\[1:0\\]
is to tell tile_rows_cnt\\[1:0\\]\n\nvp9_error_info0\\[3:2\\]
is to tell the error type"]
pub type Vp9ErrorInfo0W<'a, REG> = crate::FieldWriter<'a, REG, 32, Vp9ErrorInfo0>;
impl<'a, REG> Vp9ErrorInfo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "no error"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Vp9ErrorInfo0::B00)
    }
    #[doc = "tilesize error"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Vp9ErrorInfo0::B01)
    }
    #[doc = "seg skip error"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Vp9ErrorInfo0::B10)
    }
    #[doc = "ref scale error"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Vp9ErrorInfo0::B11)
    }
}
impl R {
    #[doc = "Bits 0:31 - vp9_error_info0\n\nfor vp9 16 tile cols, every cols contains 4bits\n\nvp9_error_info0\\[3:0\\]
is for col 0\n\nvp9_error_info0\\[1:0\\]
is to tell tile_rows_cnt\\[1:0\\]\n\nvp9_error_info0\\[3:2\\]
is to tell the error type"]
    #[inline(always)]
    pub fn vp9_error_info0(&self) -> Vp9ErrorInfo0R {
        Vp9ErrorInfo0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - vp9_error_info0\n\nfor vp9 16 tile cols, every cols contains 4bits\n\nvp9_error_info0\\[3:0\\]
is for col 0\n\nvp9_error_info0\\[1:0\\]
is to tell tile_rows_cnt\\[1:0\\]\n\nvp9_error_info0\\[3:2\\]
is to tell the error type"]
    #[inline(always)]
    #[must_use]
    pub fn vp9_error_info0(&mut self) -> Vp9ErrorInfo0W<Swreg45Vp9ErrorInfo0Spec> {
        Vp9ErrorInfo0W::new(self, 0)
    }
}
#[doc = "vp9 error info\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg45_vp9_error_info0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg45_vp9_error_info0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg45Vp9ErrorInfo0Spec;
impl crate::RegisterSpec for Swreg45Vp9ErrorInfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg45_vp9_error_info0::R`](R) reader structure"]
impl crate::Readable for Swreg45Vp9ErrorInfo0Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg45_vp9_error_info0::W`](W) writer structure"]
impl crate::Writable for Swreg45Vp9ErrorInfo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG45_VP9_ERROR_INFO0 to value 0"]
impl crate::Resettable for Swreg45Vp9ErrorInfo0Spec {
    const RESET_VALUE: u32 = 0;
}
