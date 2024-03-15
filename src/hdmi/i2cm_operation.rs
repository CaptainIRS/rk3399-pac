#[doc = "Register `I2CM_OPERATION` writer"]
pub type W = crate::W<I2cmOperationSpec>;
#[doc = "Field `RD` writer - Single byte read operation request"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_EXT` writer - After writing 1'b1 to rd_ext bit a extended data read operation is started (E-DDC read operation)."]
pub type RdExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD8` writer - Sequential read operation request. Eight bytes are read starting at the address defined in the i2cm_address.address register field and stored in the i2cm_read_buffx registers."]
pub type Rd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD8_EXT` writer - Extended sequential read operation request. Eight bytes are read starting at the address defined in register field i2cm_address.address and stored in registers i2cm_read_buffx."]
pub type Rd8ExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` writer - Single byte write operation request."]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSCLEAR` writer - Bus clear operation request."]
pub type BusclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Single byte read operation request"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<I2cmOperationSpec> {
        RdW::new(self, 0)
    }
    #[doc = "Bit 1 - After writing 1'b1 to rd_ext bit a extended data read operation is started (E-DDC read operation)."]
    #[inline(always)]
    #[must_use]
    pub fn rd_ext(&mut self) -> RdExtW<I2cmOperationSpec> {
        RdExtW::new(self, 1)
    }
    #[doc = "Bit 2 - Sequential read operation request. Eight bytes are read starting at the address defined in the i2cm_address.address register field and stored in the i2cm_read_buffx registers."]
    #[inline(always)]
    #[must_use]
    pub fn rd8(&mut self) -> Rd8W<I2cmOperationSpec> {
        Rd8W::new(self, 2)
    }
    #[doc = "Bit 3 - Extended sequential read operation request. Eight bytes are read starting at the address defined in register field i2cm_address.address and stored in registers i2cm_read_buffx."]
    #[inline(always)]
    #[must_use]
    pub fn rd8_ext(&mut self) -> Rd8ExtW<I2cmOperationSpec> {
        Rd8ExtW::new(self, 3)
    }
    #[doc = "Bit 4 - Single byte write operation request."]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WrW<I2cmOperationSpec> {
        WrW::new(self, 4)
    }
    #[doc = "Bit 5 - Bus clear operation request."]
    #[inline(always)]
    #[must_use]
    pub fn busclear(&mut self) -> BusclearW<I2cmOperationSpec> {
        BusclearW::new(self, 5)
    }
}
#[doc = "Single byte read operation request\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_operation::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmOperationSpec;
impl crate::RegisterSpec for I2cmOperationSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`i2cm_operation::W`](W) writer structure"]
impl crate::Writable for I2cmOperationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_OPERATION to value 0"]
impl crate::Resettable for I2cmOperationSpec {
    const RESET_VALUE: u8 = 0;
}
