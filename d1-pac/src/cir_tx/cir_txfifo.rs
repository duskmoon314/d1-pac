#[doc = "Register `CIR_TXFIFO` reader"]
pub struct R(crate::R<CIR_TXFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TXFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TXFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TXFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR_TXFIFO` writer"]
pub struct W(crate::W<CIR_TXFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TXFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CIR_TXFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TXFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBF` writer - Transmit Byte FIFO\n\nWhen the transmission is triggered, the data in the FIFO will be transmitted until the data number is transmitted completely."]
pub struct TBF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Byte FIFO\n\nWhen the transmission is triggered, the data in the FIFO will be transmitted until the data number is transmitted completely."]
    #[inline(always)]
    pub fn tbf(&mut self) -> TBF_W {
        TBF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit FIFO Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_txfifo](index.html) module"]
pub struct CIR_TXFIFO_SPEC;
impl crate::RegisterSpec for CIR_TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_txfifo::R](R) reader structure"]
impl crate::Readable for CIR_TXFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_txfifo::W](W) writer structure"]
impl crate::Writable for CIR_TXFIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR_TXFIFO to value 0"]
impl crate::Resettable for CIR_TXFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
